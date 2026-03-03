<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Answer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\FAQPage;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Question;
use PHPUnit\Framework\TestCase;

final class FAQPageTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new FAQPage(
			mainEntity: [
				new Question(
					name: 'What is the return policy?',
					acceptedAnswer: new Answer(text: 'You can return items within 30 days.'),
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('FAQPage', $obj->{'@type'});
		$this->assertCount(1, $obj->mainEntity);
		$this->assertEquals('Question', $obj->mainEntity[0]->{'@type'});
		$this->assertEquals('What is the return policy?', $obj->mainEntity[0]->name);
		$this->assertEquals('Answer', $obj->mainEntity[0]->acceptedAnswer->{'@type'});
		$this->assertEquals('You can return items within 30 days.', $obj->mainEntity[0]->acceptedAnswer->text);
	}

	public function testMultipleQuestionsSerializeCorrectly(): void {
		$schema = new FAQPage(
			mainEntity: [
				new Question(
					name: 'What is the return policy?',
					acceptedAnswer: new Answer(text: 'You can return items within 30 days.'),
				),
				new Question(
					name: 'Do you ship internationally?',
					acceptedAnswer: new Answer(text: 'Yes, we ship to over 50 countries.'),
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertCount(2, $obj->mainEntity);
		$this->assertEquals('What is the return policy?', $obj->mainEntity[0]->name);
		$this->assertEquals('Do you ship internationally?', $obj->mainEntity[1]->name);
		$this->assertEquals('Yes, we ship to over 50 countries.', $obj->mainEntity[1]->acceptedAnswer->text);
	}

	public function testSingleQuestionFaq(): void {
		$schema = new FAQPage(
			mainEntity: [
				new Question(
					name: 'How long does shipping take?',
					acceptedAnswer: new Answer(text: 'Shipping takes 2-4 business days.'),
				),
			],
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $schema));

		$this->assertCount(1, $obj->mainEntity);
		$this->assertEquals('How long does shipping take?', $obj->mainEntity[0]->name);
		$this->assertEquals('Shipping takes 2-4 business days.', $obj->mainEntity[0]->acceptedAnswer->text);
	}

	public function testAcceptedAnswerTextAllowsHtml(): void {
		$schema = new FAQPage(
			mainEntity: [
				new Question(
					name: 'Can I use formatting in answers?',
					acceptedAnswer: new Answer(text: '<p>Yes, use <strong>HTML</strong> when needed.</p>'),
				),
			],
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $schema));

		$this->assertEquals('<p>Yes, use <strong>HTML</strong> when needed.</p>', $obj->mainEntity[0]->acceptedAnswer->text);
	}

	public function testLargeFaqWithFiveQuestions(): void {
		$schema = new FAQPage(
			mainEntity: [
				new Question(name: 'Q1', acceptedAnswer: new Answer(text: 'A1')),
				new Question(name: 'Q2', acceptedAnswer: new Answer(text: 'A2')),
				new Question(name: 'Q3', acceptedAnswer: new Answer(text: 'A3')),
				new Question(name: 'Q4', acceptedAnswer: new Answer(text: 'A4')),
				new Question(name: 'Q5', acceptedAnswer: new Answer(text: 'A5')),
			],
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $schema));

		$this->assertCount(5, $obj->mainEntity);
		$this->assertEquals('Q1', $obj->mainEntity[0]->name);
		$this->assertEquals('A5', $obj->mainEntity[4]->acceptedAnswer->text);
	}
}
