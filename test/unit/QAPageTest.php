<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Answer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use EvaLok\SchemaOrgJsonLd\v1\Schema\QAPage;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Question;
use PHPUnit\Framework\TestCase;

final class QAPageTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new QAPage(
			mainEntity: new Question(
				name: 'How do I reset my password?',
				acceptedAnswer: new Answer(text: 'Click the forgot password link on the login page.'),
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('QAPage', $obj->{'@type'});
		$this->assertEquals('Question', $obj->mainEntity->{'@type'});
		$this->assertEquals('How do I reset my password?', $obj->mainEntity->name);
		$this->assertEquals('Answer', $obj->mainEntity->acceptedAnswer->{'@type'});
		$this->assertEquals('Click the forgot password link on the login page.', $obj->mainEntity->acceptedAnswer->text);
	}

	public function testQAPageWithSuggestedAnswers(): void {
		$schema = new QAPage(
			mainEntity: new Question(
				name: 'What is the best PHP framework?',
				suggestedAnswer: [
					new Answer(text: 'Laravel is very popular.'),
					new Answer(text: 'Symfony is very flexible.'),
				],
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('QAPage', $obj->{'@type'});
		$this->assertEquals('Question', $obj->mainEntity->{'@type'});
		$this->assertCount(2, $obj->mainEntity->suggestedAnswer);
		$this->assertEquals('Answer', $obj->mainEntity->suggestedAnswer[0]->{'@type'});
		$this->assertEquals('Laravel is very popular.', $obj->mainEntity->suggestedAnswer[0]->text);
		$this->assertEquals('Symfony is very flexible.', $obj->mainEntity->suggestedAnswer[1]->text);
	}

	public function testFullQAPage(): void {
		$author = new Person(name: 'Jane Doe');
		$schema = new QAPage(
			mainEntity: new Question(
				name: 'How do I reset my password?',
				acceptedAnswer: new Answer(text: 'Click the forgot password link on the login page.'),
				suggestedAnswer: [
					new Answer(text: 'You can also contact support.'),
				],
				answerCount: 2,
				text: 'I cannot find the reset password option anywhere on the site.',
				upvoteCount: 42,
				author: $author,
				datePublished: '2024-01-15',
				dateModified: '2024-02-01',
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('QAPage', $obj->{'@type'});
		$q = $obj->mainEntity;
		$this->assertEquals('How do I reset my password?', $q->name);
		$this->assertEquals('Answer', $q->acceptedAnswer->{'@type'});
		$this->assertCount(1, $q->suggestedAnswer);
		$this->assertEquals(2, $q->answerCount);
		$this->assertEquals('I cannot find the reset password option anywhere on the site.', $q->text);
		$this->assertEquals(42, $q->upvoteCount);
		$this->assertEquals('Person', $q->author->{'@type'});
		$this->assertEquals('Jane Doe', $q->author->name);
		$this->assertEquals('2024-01-15', $q->datePublished);
		$this->assertEquals('2024-02-01', $q->dateModified);
	}

	public function testRichAnswer(): void {
		$author = new Organization(name: 'Acme Corp');
		$schema = new Answer(
			text: 'Click the forgot password link.',
			author: $author,
			url: 'https://example.com/answer/1',
			upvoteCount: 15,
			datePublished: '2024-01-10',
			dateModified: '2024-01-20',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Answer', $obj->{'@type'});
		$this->assertEquals('Click the forgot password link.', $obj->text);
		$this->assertEquals('Organization', $obj->author->{'@type'});
		$this->assertEquals('Acme Corp', $obj->author->name);
		$this->assertEquals('https://example.com/answer/1', $obj->url);
		$this->assertEquals(15, $obj->upvoteCount);
		$this->assertEquals('2024-01-10', $obj->datePublished);
		$this->assertEquals('2024-01-20', $obj->dateModified);
	}

	public function testNullFieldsOmitted(): void {
		$schema = new QAPage(
			mainEntity: new Question(
				name: 'Simple question?',
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$q = $obj->mainEntity;
		$this->assertObjectNotHasProperty('acceptedAnswer', $q);
		$this->assertObjectNotHasProperty('suggestedAnswer', $q);
		$this->assertObjectNotHasProperty('answerCount', $q);
		$this->assertObjectNotHasProperty('text', $q);
		$this->assertObjectNotHasProperty('upvoteCount', $q);
		$this->assertObjectNotHasProperty('author', $q);
		$this->assertObjectNotHasProperty('datePublished', $q);
		$this->assertObjectNotHasProperty('dateModified', $q);

		$answer = new Answer(text: 'Some answer.');
		$answerJson = JsonLdGenerator::SchemaToJson(schema: $answer);
		$answerObj = json_decode($answerJson);
		$this->assertObjectNotHasProperty('author', $answerObj);
		$this->assertObjectNotHasProperty('url', $answerObj);
		$this->assertObjectNotHasProperty('upvoteCount', $answerObj);
		$this->assertObjectNotHasProperty('datePublished', $answerObj);
		$this->assertObjectNotHasProperty('dateModified', $answerObj);
	}
}
