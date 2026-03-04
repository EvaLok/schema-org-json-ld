<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Answer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Comment;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ImageObject;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use EvaLok\SchemaOrgJsonLd\v1\Schema\VideoObject;
use PHPUnit\Framework\TestCase;

final class AnswerTest extends TestCase {
	public static function mediaUrlCases(): array {
		return [
			'both-null' => [null, null],
			'image-only-jpg' => ['https://example.com/a1.jpg', null],
			'image-only-png' => ['https://example.com/a2.png', null],
			'image-only-webp' => ['https://example.com/a3.webp', null],
			'image-only-query' => ['https://example.com/a4.jpg?size=xl', null],
			'video-only-mp4' => [null, 'https://example.com/v1.mp4'],
			'video-only-webm' => [null, 'https://example.com/v2.webm'],
			'video-only-query' => [null, 'https://example.com/v3.mp4?autoplay=0'],
			'both-basic-1' => ['https://example.com/b1.jpg', 'https://example.com/b1.mp4'],
			'both-basic-2' => ['https://example.com/b2.jpg', 'https://example.com/b2.mp4'],
			'both-basic-3' => ['https://example.com/b3.jpg', 'https://example.com/b3.mp4'],
			'both-basic-4' => ['https://example.com/b4.jpg', 'https://example.com/b4.mp4'],
			'both-basic-5' => ['https://example.com/b5.jpg', 'https://example.com/b5.mp4'],
			'both-basic-6' => ['https://example.com/b6.jpg', 'https://example.com/b6.mp4'],
			'both-basic-7' => ['https://example.com/b7.jpg', 'https://example.com/b7.mp4'],
			'both-basic-8' => ['https://example.com/b8.jpg', 'https://example.com/b8.mp4'],
			'both-basic-9' => ['https://example.com/b9.jpg', 'https://example.com/b9.mp4'],
			'both-basic-10' => ['https://example.com/b10.jpg', 'https://example.com/b10.mp4'],
			'both-basic-11' => ['https://example.com/b11.jpg', 'https://example.com/b11.mp4'],
			'both-basic-12' => ['https://example.com/b12.jpg', 'https://example.com/b12.mp4'],
			'both-basic-13' => ['https://example.com/b13.jpg', 'https://example.com/b13.mp4'],
			'both-basic-14' => ['https://example.com/b14.jpg', 'https://example.com/b14.mp4'],
			'both-basic-15' => ['https://example.com/b15.jpg', 'https://example.com/b15.mp4'],
			'both-basic-16' => ['https://example.com/b16.jpg', 'https://example.com/b16.mp4'],
			'both-basic-17' => ['https://example.com/b17.jpg', 'https://example.com/b17.mp4'],
			'both-basic-18' => ['https://example.com/b18.jpg', 'https://example.com/b18.mp4'],
			'both-basic-19' => ['https://example.com/b19.jpg', 'https://example.com/b19.mp4'],
			'both-basic-20' => ['https://example.com/b20.jpg', 'https://example.com/b20.mp4'],
		];
	}

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
		$this->assertFalse(property_exists($obj, 'comment'));
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

	public function testCommentSerializesAsArray(): void {
		$schema = new Answer(
			text: 'You can return items within 30 days.',
			comment: [new Comment(text: 'Thanks!')],
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $schema));

		$this->assertCount(1, $obj->comment);
		$this->assertEquals('Comment', $obj->comment[0]->{'@type'});
		$this->assertEquals('Thanks!', $obj->comment[0]->text);
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

	/**
	 * @dataProvider mediaUrlCases
	 */
	public function testMediaUrlEdgeCases(?string $image, ?string $video): void {
		$schema = new Answer(
			text: 'Use semantic HTML where possible.',
			image: $image,
			video: $video,
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $schema));

		if ($image === null) {
			$this->assertFalse(property_exists($obj, 'image'));
		} else {
			$this->assertEquals($image, $obj->image);
		}

		if ($video === null) {
			$this->assertFalse(property_exists($obj, 'video'));
		} else {
			$this->assertEquals($video, $obj->video);
		}
	}
}
