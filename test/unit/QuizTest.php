<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AlignmentObject;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Answer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Question;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Quiz;
use PHPUnit\Framework\TestCase;

final class QuizTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new Quiz(
			hasPart: [
				new Question(
					name: 'What is 2 + 2?',
					acceptedAnswer: new Answer(text: '4'),
					eduQuestionType: 'Flashcard',
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Quiz', $obj->{'@type'});
		$this->assertCount(1, $obj->hasPart);
		$this->assertEquals('Question', $obj->hasPart[0]->{'@type'});
		$this->assertEquals('What is 2 + 2?', $obj->hasPart[0]->name);
		$this->assertEquals('Flashcard', $obj->hasPart[0]->eduQuestionType);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new Quiz(
			hasPart: [
				new Question(name: 'What is 2 + 2?'),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'about'));
		$this->assertFalse(property_exists($obj, 'educationalAlignment'));
		$this->assertFalse(property_exists($obj, 'name'));
		$this->assertFalse(property_exists($obj, 'description'));
	}

	public function testFullOutput(): void {
		$schema = new Quiz(
			hasPart: [
				new Question(
					name: 'What is 2 + 2?',
					acceptedAnswer: new Answer(text: '4'),
					eduQuestionType: 'Flashcard',
				),
			],
			about: 'Addition',
			educationalAlignment: new AlignmentObject(
				alignmentType: 'educationalSubject',
				targetName: 'Mathematics',
				educationalFramework: 'Example Curriculum',
				targetUrl: 'https://example.org/framework/math',
			),
			name: 'Basic Addition Flashcards',
			description: 'Simple flashcards for addition practice.',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Addition', $obj->about);
		$this->assertEquals('AlignmentObject', $obj->educationalAlignment->{'@type'});
		$this->assertEquals('educationalSubject', $obj->educationalAlignment->alignmentType);
		$this->assertEquals('Mathematics', $obj->educationalAlignment->targetName);
		$this->assertEquals('Basic Addition Flashcards', $obj->name);
		$this->assertEquals('Simple flashcards for addition practice.', $obj->description);
	}
}
