<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ImageObject;
use EvaLok\SchemaOrgJsonLd\v1\Schema\NewsArticle;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use PHPUnit\Framework\TestCase;

final class NewsArticleTest extends TestCase {
	public function testMinimalOutput(): void {
		$article = new NewsArticle(headline: 'Breaking news');
		$json = JsonLdGenerator::SchemaToJson(schema: $article);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('NewsArticle', $obj->{'@type'});
		$this->assertEquals('Breaking news', $obj->headline);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$article = new NewsArticle(headline: 'Breaking news');
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
		$article = new NewsArticle(
			headline: 'Breaking news',
			author: new Person(name: 'Reporter'),
			datePublished: '2026-02-24T17:00:00+00:00',
			dateModified: '2026-02-24T18:00:00+00:00',
			image: [
				new ImageObject(contentUrl: 'https://example.com/news.jpg'),
			],
			description: 'News article description',
			publisher: new Organization(name: 'News Publisher'),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $article);
		$obj = json_decode($json);

		$this->assertEquals('Breaking news', $obj->headline);
		$this->assertEquals('Person', $obj->author->{'@type'});
		$this->assertEquals('Reporter', $obj->author->name);
		$this->assertEquals('2026-02-24T17:00:00+00:00', $obj->datePublished);
		$this->assertEquals('2026-02-24T18:00:00+00:00', $obj->dateModified);
		$this->assertEquals('ImageObject', $obj->image[0]->{'@type'});
		$this->assertEquals('https://example.com/news.jpg', $obj->image[0]->contentUrl);
		$this->assertEquals('News article description', $obj->description);
		$this->assertEquals('Organization', $obj->publisher->{'@type'});
		$this->assertEquals('News Publisher', $obj->publisher->name);
	}
}
