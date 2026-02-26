<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Article;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ImageObject;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use PHPUnit\Framework\TestCase;

final class ArticleTest extends TestCase {
	public function testMinimalOutput(): void {
		$article = new Article(headline: 'Test article title');
		$json = JsonLdGenerator::SchemaToJson(schema: $article);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Article', $obj->{'@type'});
		$this->assertEquals('Test article title', $obj->headline);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$article = new Article(headline: 'Test article title');
		$json = JsonLdGenerator::SchemaToJson(schema: $article);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'author'));
		$this->assertFalse(property_exists($obj, 'datePublished'));
		$this->assertFalse(property_exists($obj, 'dateModified'));
		$this->assertFalse(property_exists($obj, 'image'));
		$this->assertFalse(property_exists($obj, 'description'));
		$this->assertFalse(property_exists($obj, 'publisher'));
	}

	public function testFullOutputWithNestedTypes(): void {
		$article = new Article(
			headline: 'Test article title',
			author: [
				new Person(name: 'John Doe'),
				new Organization(name: 'Editorial Team'),
			],
			datePublished: '2026-02-24T17:00:00+00:00',
			dateModified: '2026-02-24T18:00:00+00:00',
			image: [
				new ImageObject(contentUrl: 'https://example.com/image-a.jpg'),
				new ImageObject(contentUrl: 'https://example.com/image-b.jpg', caption: 'Image B'),
			],
			description: 'Test article description',
			publisher: new Organization(name: 'Example Publisher'),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $article);
		$obj = json_decode($json);

		$this->assertEquals('Test article title', $obj->headline);
		$this->assertEquals('Person', $obj->author[0]->{'@type'});
		$this->assertEquals('John Doe', $obj->author[0]->name);
		$this->assertEquals('Organization', $obj->author[1]->{'@type'});
		$this->assertEquals('Editorial Team', $obj->author[1]->name);
		$this->assertEquals('2026-02-24T17:00:00+00:00', $obj->datePublished);
		$this->assertEquals('2026-02-24T18:00:00+00:00', $obj->dateModified);
		$this->assertEquals('ImageObject', $obj->image[0]->{'@type'});
		$this->assertEquals('https://example.com/image-a.jpg', $obj->image[0]->contentUrl);
		$this->assertEquals('ImageObject', $obj->image[1]->{'@type'});
		$this->assertEquals('https://example.com/image-b.jpg', $obj->image[1]->contentUrl);
		$this->assertEquals('Image B', $obj->image[1]->caption);
		$this->assertEquals('Test article description', $obj->description);
		$this->assertEquals('Organization', $obj->publisher->{'@type'});
		$this->assertEquals('Example Publisher', $obj->publisher->name);
	}
}
