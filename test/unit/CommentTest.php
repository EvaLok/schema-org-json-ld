<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Comment;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ImageObject;
use EvaLok\SchemaOrgJsonLd\v1\Schema\InteractionCounter;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use EvaLok\SchemaOrgJsonLd\v1\Schema\VideoObject;
use PHPUnit\Framework\TestCase;

final class CommentTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new Comment(text: 'Nice article!');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Comment', $obj->{'@type'});
		$this->assertEquals('Nice article!', $obj->text);
	}

	public function testFullOutput(): void {
		$schema = new Comment(
			text: 'Nice article!',
			author: new Person(name: 'Jane Doe'),
			datePublished: '2024-03-01T09:46:02+02:00',
			url: 'https://example.com/post#comment-1',
			dateModified: '2024-03-01T10:00:00+02:00',
			image: new ImageObject(contentUrl: 'https://example.com/comment-image.jpg'),
			video: new VideoObject(
				name: 'Comment video',
				thumbnailUrl: ['https://example.com/comment-thumb.jpg'],
				uploadDate: '2024-03-01T09:00:00+02:00',
			),
			comment: [
				new Comment(text: 'Nested reply'),
			],
			interactionStatistic: new InteractionCounter(
				interactionType: 'https://schema.org/LikeAction',
				userInteractionCount: 3,
			),
			sharedContent: 'https://example.com/shared-comment-content',
			creativeWorkStatus: 'Deleted',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Person', $obj->author->{'@type'});
		$this->assertEquals('Jane Doe', $obj->author->name);
		$this->assertEquals('2024-03-01T09:46:02+02:00', $obj->datePublished);
		$this->assertEquals('https://example.com/post#comment-1', $obj->url);
		$this->assertEquals('2024-03-01T10:00:00+02:00', $obj->dateModified);
		$this->assertEquals('ImageObject', $obj->image->{'@type'});
		$this->assertEquals('VideoObject', $obj->video->{'@type'});
		$this->assertCount(1, $obj->comment);
		$this->assertEquals('Nested reply', $obj->comment[0]->text);
		$this->assertEquals('InteractionCounter', $obj->interactionStatistic->{'@type'});
		$this->assertEquals('https://example.com/shared-comment-content', $obj->sharedContent);
		$this->assertEquals('Deleted', $obj->creativeWorkStatus);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new Comment(text: 'Nice article!');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'author'));
		$this->assertFalse(property_exists($obj, 'datePublished'));
		$this->assertFalse(property_exists($obj, 'url'));
		$this->assertFalse(property_exists($obj, 'dateModified'));
		$this->assertFalse(property_exists($obj, 'image'));
		$this->assertFalse(property_exists($obj, 'video'));
		$this->assertFalse(property_exists($obj, 'comment'));
		$this->assertFalse(property_exists($obj, 'interactionStatistic'));
		$this->assertFalse(property_exists($obj, 'sharedContent'));
		$this->assertFalse(property_exists($obj, 'creativeWorkStatus'));
	}

	public function testInteractionStatisticSerializesAsArray(): void {
		$schema = new Comment(
			text: 'Nice article!',
			interactionStatistic: [
				new InteractionCounter(
					interactionType: 'https://schema.org/LikeAction',
					userInteractionCount: 3,
				),
				new InteractionCounter(
					interactionType: 'https://schema.org/CommentAction',
					userInteractionCount: 1,
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertCount(2, $obj->interactionStatistic);
		$this->assertEquals('InteractionCounter', $obj->interactionStatistic[0]->{'@type'});
		$this->assertEquals('https://schema.org/LikeAction', $obj->interactionStatistic[0]->interactionType);
		$this->assertEquals('https://schema.org/CommentAction', $obj->interactionStatistic[1]->interactionType);
	}
}
