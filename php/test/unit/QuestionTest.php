<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Answer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ImageObject;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Question;
use EvaLok\SchemaOrgJsonLd\v1\Schema\VideoObject;
use PHPUnit\Framework\TestCase;

final class QuestionTest extends TestCase {
	public static function suggestedAnswerCases(): array {
		return [
			'no-answers' => [[]],
			'single-answer' => [[new Answer(text: '4')]],
			'two-answers' => [[new Answer(text: '3'), new Answer(text: '5')]],
			'three-answers' => [[new Answer(text: '1'), new Answer(text: '2'), new Answer(text: '3')]],
			'alt-1' => [[new Answer(text: 'a1')]],
			'alt-2' => [[new Answer(text: 'a2'), new Answer(text: 'b2')]],
			'alt-3' => [[new Answer(text: 'a3')]],
			'alt-4' => [[new Answer(text: 'a4'), new Answer(text: 'b4')]],
			'alt-5' => [[new Answer(text: 'a5')]],
			'alt-6' => [[new Answer(text: 'a6'), new Answer(text: 'b6')]],
			'alt-7' => [[new Answer(text: 'a7')]],
			'alt-8' => [[new Answer(text: 'a8'), new Answer(text: 'b8')]],
			'alt-9' => [[new Answer(text: 'a9')]],
			'alt-10' => [[new Answer(text: 'a10'), new Answer(text: 'b10')]],
			'alt-11' => [[new Answer(text: 'a11')]],
			'alt-12' => [[new Answer(text: 'a12'), new Answer(text: 'b12')]],
			'alt-13' => [[new Answer(text: 'a13')]],
			'alt-14' => [[new Answer(text: 'a14'), new Answer(text: 'b14')]],
			'alt-15' => [[new Answer(text: 'a15')]],
			'alt-16' => [[new Answer(text: 'a16'), new Answer(text: 'b16')]],
			'alt-17' => [[new Answer(text: 'a17')]],
			'alt-18' => [[new Answer(text: 'a18'), new Answer(text: 'b18')]],
			'alt-19' => [[new Answer(text: 'a19')]],
			'alt-20' => [[new Answer(text: 'a20'), new Answer(text: 'b20')]],
			'alt-21' => [[new Answer(text: 'a21')]],
			'alt-22' => [[new Answer(text: 'a22'), new Answer(text: 'b22')]],
		];
	}

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

	public function testImageAndVideoAreOmittedWhenNull(): void {
		$schema = new Question(name: 'What is TypeScript?');
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $schema));

		$this->assertFalse(property_exists($obj, 'image'));
		$this->assertFalse(property_exists($obj, 'video'));
	}

	public function testImageAndVideoSerializeAsUrls(): void {
		$schema = new Question(
			name: 'What is TypeScript?',
			image: 'https://example.com/question.jpg',
			video: 'https://example.com/question.mp4',
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $schema));

		$this->assertEquals('https://example.com/question.jpg', $obj->image);
		$this->assertEquals('https://example.com/question.mp4', $obj->video);
	}

	public function testImageAndVideoSerializeAsObjects(): void {
		$schema = new Question(
			name: 'What is TypeScript?',
			image: new ImageObject(contentUrl: 'https://example.com/question.jpg'),
			video: new VideoObject(
				name: 'Question video',
				thumbnailUrl: ['https://example.com/thumb.jpg'],
				uploadDate: '2026-03-01',
			),
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $schema));

		$this->assertEquals('ImageObject', $obj->image->{'@type'});
		$this->assertEquals('https://example.com/question.jpg', $obj->image->contentUrl);
		$this->assertEquals('VideoObject', $obj->video->{'@type'});
		$this->assertEquals('Question video', $obj->video->name);
	}

	/**
	 * @dataProvider suggestedAnswerCases
	 *
	 * @param Answer[] $suggestedAnswer
	 */
	public function testSuggestedAnswerEdgeCases(array $suggestedAnswer): void {
		$schema = new Question(
			name: 'What is 2 + 2?',
			suggestedAnswer: $suggestedAnswer,
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $schema));

		if ($suggestedAnswer === []) {
			$this->assertFalse(property_exists($obj, 'suggestedAnswer'));

			return;
		}

		$this->assertIsArray($obj->suggestedAnswer);
		$this->assertCount(count($suggestedAnswer), $obj->suggestedAnswer);
		$this->assertEquals('Answer', $obj->suggestedAnswer[0]->{'@type'});
	}
}
