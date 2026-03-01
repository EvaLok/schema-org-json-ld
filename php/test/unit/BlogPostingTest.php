<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\BlogPosting;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ImageObject;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use PHPUnit\Framework\TestCase;

final class BlogPostingTest extends TestCase {
	public function testMinimalOutput(): void {
		$post = new BlogPosting(headline: 'Blog title');
		$json = JsonLdGenerator::SchemaToJson(schema: $post);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('BlogPosting', $obj->{'@type'});
		$this->assertEquals('Blog title', $obj->headline);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$post = new BlogPosting(headline: 'Blog title');
		$json = JsonLdGenerator::SchemaToJson(schema: $post);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'author'));
		$this->assertFalse(property_exists($obj, 'datePublished'));
		$this->assertFalse(property_exists($obj, 'dateModified'));
		$this->assertFalse(property_exists($obj, 'image'));
		$this->assertFalse(property_exists($obj, 'description'));
		$this->assertFalse(property_exists($obj, 'publisher'));
	}

	public function testFullOutputWithNestedTypes(): void {
		$post = new BlogPosting(
			headline: 'Blog title',
			author: new Organization(name: 'Editorial Blog Team'),
			datePublished: '2026-02-24T17:00:00+00:00',
			dateModified: '2026-02-24T18:00:00+00:00',
			image: [
				new ImageObject(contentUrl: 'https://example.com/blog-wide.jpg'),
			],
			description: 'Blog description',
			publisher: new Organization(name: 'Blog Publisher'),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $post);
		$obj = json_decode($json);

		$this->assertEquals('Blog title', $obj->headline);
		$this->assertEquals('Organization', $obj->author->{'@type'});
		$this->assertEquals('Editorial Blog Team', $obj->author->name);
		$this->assertEquals('2026-02-24T17:00:00+00:00', $obj->datePublished);
		$this->assertEquals('2026-02-24T18:00:00+00:00', $obj->dateModified);
		$this->assertEquals('ImageObject', $obj->image[0]->{'@type'});
		$this->assertEquals('https://example.com/blog-wide.jpg', $obj->image[0]->contentUrl);
		$this->assertEquals('Blog description', $obj->description);
		$this->assertEquals('Organization', $obj->publisher->{'@type'});
		$this->assertEquals('Blog Publisher', $obj->publisher->name);
	}
}
