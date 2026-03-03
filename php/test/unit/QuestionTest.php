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
}
