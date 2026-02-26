<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Answer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Question;
use PHPUnit\Framework\TestCase;

final class QuestionTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new Question(
			name: 'What is the return policy?',
			acceptedAnswer: new Answer(text: 'You can return items within 30 days.'),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Question', $obj->{'@type'});
		$this->assertEquals('What is the return policy?', $obj->name);
		$this->assertEquals('Answer', $obj->acceptedAnswer->{'@type'});
		$this->assertEquals('You can return items within 30 days.', $obj->acceptedAnswer->text);
	}

	public function testEduQuestionTypeIsSerializedWhenProvided(): void {
		$schema = new Question(
			name: 'What is 2 + 2?',
			eduQuestionType: 'Flashcard',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Flashcard', $obj->eduQuestionType);
	}
}
