<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Answer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ImageObject;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use EvaLok\SchemaOrgJsonLd\v1\Schema\VideoObject;
use PHPUnit\Framework\TestCase;

final class AnswerTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new Answer(text: 'You can return items within 30 days.');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Answer', $obj->{'@type'});
		$this->assertEquals('You can return items within 30 days.', $obj->text);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new Answer(text: 'You can return items within 30 days.');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'author'));
		$this->assertFalse(property_exists($obj, 'url'));
		$this->assertFalse(property_exists($obj, 'image'));
		$this->assertFalse(property_exists($obj, 'video'));
		$this->assertFalse(property_exists($obj, 'upvoteCount'));
		$this->assertFalse(property_exists($obj, 'datePublished'));
		$this->assertFalse(property_exists($obj, 'dateModified'));
	}

	public function testFullOutput(): void {
		$schema = new Answer(
			text: 'You can return items within 30 days.',
			author: new Person(name: 'John'),
			url: 'https://example.com/faq/returns',
			upvoteCount: 42,
			datePublished: '2026-02-25',
			dateModified: '2026-02-26',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('You can return items within 30 days.', $obj->text);
		$this->assertEquals('Person', $obj->author->{'@type'});
		$this->assertEquals('John', $obj->author->name);
		$this->assertEquals('https://example.com/faq/returns', $obj->url);
		$this->assertEquals(42, $obj->upvoteCount);
		$this->assertEquals('2026-02-25', $obj->datePublished);
		$this->assertEquals('2026-02-26', $obj->dateModified);
	}

	public function testImageAndVideoSerializeAsUrls(): void {
		$schema = new Answer(
			text: 'Use semantic HTML where possible.',
			image: 'https://example.com/answer.jpg',
			video: 'https://example.com/answer.mp4',
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $schema));

		$this->assertEquals('https://example.com/answer.jpg', $obj->image);
		$this->assertEquals('https://example.com/answer.mp4', $obj->video);
	}

	public function testImageAndVideoSerializeAsObjects(): void {
		$schema = new Answer(
			text: 'Use semantic HTML where possible.',
			image: new ImageObject(contentUrl: 'https://example.com/answer.jpg'),
			video: new VideoObject(
				name: 'Answer video',
				thumbnailUrl: ['https://example.com/thumb.jpg'],
				uploadDate: '2026-03-01',
			),
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $schema));

		$this->assertEquals('ImageObject', $obj->image->{'@type'});
		$this->assertEquals('https://example.com/answer.jpg', $obj->image->contentUrl);
		$this->assertEquals('VideoObject', $obj->video->{'@type'});
		$this->assertEquals('Answer video', $obj->video->name);
	}
}
