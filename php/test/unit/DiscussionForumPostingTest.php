<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Comment;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DiscussionForumPosting;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ImageObject;
use EvaLok\SchemaOrgJsonLd\v1\Schema\InteractionCounter;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use EvaLok\SchemaOrgJsonLd\v1\Schema\VideoObject;
use PHPUnit\Framework\TestCase;

final class DiscussionForumPostingTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new DiscussionForumPosting(
			author: new Person(name: 'Author Name'),
			datePublished: '2024-03-01T08:34:34+02:00',
			text: 'Post content',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('DiscussionForumPosting', $obj->{'@type'});
		$this->assertEquals('Post content', $obj->text);
		$this->assertEquals('2024-03-01T08:34:34+02:00', $obj->datePublished);
		$this->assertEquals('Person', $obj->author->{'@type'});
		$this->assertEquals('Author Name', $obj->author->name);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new DiscussionForumPosting(
			author: new Person(name: 'Author Name'),
			datePublished: '2024-03-01T08:34:34+02:00',
			text: 'Post content',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertObjectNotHasProperty('headline', $obj);
		$this->assertObjectNotHasProperty('url', $obj);
		$this->assertObjectNotHasProperty('dateModified', $obj);
		$this->assertObjectNotHasProperty('image', $obj);
		$this->assertObjectNotHasProperty('video', $obj);
		$this->assertObjectNotHasProperty('comment', $obj);
		$this->assertObjectNotHasProperty('interactionStatistic', $obj);
		$this->assertObjectNotHasProperty('isPartOf', $obj);
		$this->assertObjectNotHasProperty('sharedContent', $obj);
		$this->assertObjectNotHasProperty('creativeWorkStatus', $obj);
		$this->assertObjectNotHasProperty('mainEntityOfPage', $obj);
	}

	public function testFullOutput(): void {
		$schema = new DiscussionForumPosting(
			author: new Person(
				name: 'Author Name',
				interactionStatistic: new InteractionCounter(
					interactionType: 'https://schema.org/FollowAction',
					userInteractionCount: 1,
				),
				agentInteractionStatistic: new InteractionCounter(
					interactionType: 'https://schema.org/WriteAction',
					userInteractionCount: 12,
				),
				identifier: 'author-1',
				alternateName: 'author_alias',
			),
			datePublished: '2024-03-01T08:34:34+02:00',
			text: 'Post content',
			headline: 'Post title',
			url: 'https://example.com/post/1',
			dateModified: '2024-03-01T08:40:00+02:00',
			image: new ImageObject(contentUrl: 'https://example.com/image.jpg'),
			video: new VideoObject(
				name: 'Inline video',
				thumbnailUrl: ['https://example.com/thumb.jpg'],
				uploadDate: '2024-03-01T08:00:00+02:00',
			),
			comment: [
				new Comment(
					text: 'A reply',
					author: new Person(name: 'Commenter'),
					datePublished: '2024-03-01T09:46:02+02:00',
					url: 'https://example.com/post/1#comment-1',
					dateModified: '2024-03-01T09:50:00+02:00',
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
				),
			],
			interactionStatistic: new InteractionCounter(
				interactionType: 'https://schema.org/LikeAction',
				userInteractionCount: 27,
			),
			isPartOf: 'https://example.com/forum',
			sharedContent: 'https://example.com/shared-post-content',
			creativeWorkStatus: 'Deleted',
			mainEntityOfPage: 'https://example.com/post/1',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Post title', $obj->headline);
		$this->assertEquals('https://example.com/post/1', $obj->url);
		$this->assertEquals('2024-03-01T08:40:00+02:00', $obj->dateModified);
		$this->assertEquals('ImageObject', $obj->image->{'@type'});
		$this->assertEquals('VideoObject', $obj->video->{'@type'});
		$this->assertEquals('InteractionCounter', $obj->interactionStatistic->{'@type'});
		$this->assertEquals('https://example.com/forum', $obj->isPartOf);
		$this->assertEquals('https://example.com/shared-post-content', $obj->sharedContent);
		$this->assertEquals('Deleted', $obj->creativeWorkStatus);
		$this->assertEquals('https://example.com/post/1', $obj->mainEntityOfPage);
		$this->assertEquals('InteractionCounter', $obj->author->interactionStatistic->{'@type'});
		$this->assertEquals('InteractionCounter', $obj->author->agentInteractionStatistic->{'@type'});
		$this->assertEquals('author-1', $obj->author->identifier);
		$this->assertEquals('author_alias', $obj->author->alternateName);
		$this->assertCount(1, $obj->comment);
		$this->assertEquals('https://example.com/post/1#comment-1', $obj->comment[0]->url);
		$this->assertEquals('2024-03-01T09:50:00+02:00', $obj->comment[0]->dateModified);
		$this->assertEquals('ImageObject', $obj->comment[0]->image->{'@type'});
		$this->assertEquals('VideoObject', $obj->comment[0]->video->{'@type'});
		$this->assertCount(1, $obj->comment[0]->comment);
		$this->assertEquals('Nested reply', $obj->comment[0]->comment[0]->text);
		$this->assertEquals('InteractionCounter', $obj->comment[0]->interactionStatistic->{'@type'});
		$this->assertEquals('https://example.com/shared-comment-content', $obj->comment[0]->sharedContent);
		$this->assertEquals('Deleted', $obj->comment[0]->creativeWorkStatus);
	}

	public function testInteractionStatisticSerializesAsArray(): void {
		$schema = new DiscussionForumPosting(
			author: new Person(name: 'Author Name'),
			datePublished: '2024-03-01T08:34:34+02:00',
			text: 'Post content',
			interactionStatistic: [
				new InteractionCounter(
					interactionType: 'https://schema.org/LikeAction',
					userInteractionCount: 27,
				),
				new InteractionCounter(
					interactionType: 'https://schema.org/ViewAction',
					userInteractionCount: 102,
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertCount(2, $obj->interactionStatistic);
		$this->assertEquals('InteractionCounter', $obj->interactionStatistic[0]->{'@type'});
		$this->assertEquals('https://schema.org/LikeAction', $obj->interactionStatistic[0]->interactionType);
		$this->assertEquals('https://schema.org/ViewAction', $obj->interactionStatistic[1]->interactionType);
	}
}
