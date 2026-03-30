<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Article;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ImageObject;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use EvaLok\SchemaOrgJsonLd\v1\Schema\SpeakableSpecification;
use EvaLok\SchemaOrgJsonLd\v1\Schema\WebPageElement;
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

	public function testSpeakableProperty(): void {
		$article = new Article(
			headline: 'Test article title',
			speakable: new SpeakableSpecification(
				cssSelector: ['.headline', '.article-body'],
				xpath: '/html/body/main/article',
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $article);
		$obj = json_decode($json);

		$this->assertEquals('SpeakableSpecification', $obj->speakable->{'@type'});
		$this->assertEquals(['.headline', '.article-body'], $obj->speakable->cssSelector);
		$this->assertEquals('/html/body/main/article', $obj->speakable->xpath);
	}

	public function testIsAccessibleForFreeBoolean(): void {
		$freeArticle = new Article(
			headline: 'Free article',
			isAccessibleForFree: true,
		);
		$freeJson = JsonLdGenerator::SchemaToJson(schema: $freeArticle);
		$freeObj = json_decode($freeJson);

		$this->assertTrue($freeObj->isAccessibleForFree);

		$subscriptionArticle = new Article(
			headline: 'Subscription article',
			isAccessibleForFree: false,
		);
		$subscriptionJson = JsonLdGenerator::SchemaToJson(schema: $subscriptionArticle);
		$subscriptionObj = json_decode($subscriptionJson);

		$this->assertFalse($subscriptionObj->isAccessibleForFree);
	}

	public function testSubscriptionContentScenario(): void {
		$article = new Article(
			headline: 'Subscription article',
			isAccessibleForFree: false,
			hasPart: [
				new WebPageElement(
					isAccessibleForFree: false,
					cssSelector: '.paywall',
				),
				new WebPageElement(
					isAccessibleForFree: false,
					cssSelector: '.subscriber-only',
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $article);
		$obj = json_decode($json);

		$this->assertFalse($obj->isAccessibleForFree);
		$this->assertCount(2, $obj->hasPart);
		$this->assertEquals('WebPageElement', $obj->hasPart[0]->{'@type'});
		$this->assertFalse($obj->hasPart[0]->isAccessibleForFree);
		$this->assertEquals('.paywall', $obj->hasPart[0]->cssSelector);
		$this->assertEquals('WebPageElement', $obj->hasPart[1]->{'@type'});
		$this->assertFalse($obj->hasPart[1]->isAccessibleForFree);
		$this->assertEquals('.subscriber-only', $obj->hasPart[1]->cssSelector);
	}

	public function testSinglePersonAuthor(): void {
		$article = new Article(
			headline: 'Single author article',
			author: new Person(name: 'John Doe'),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $article);
		$obj = json_decode($json);

		$this->assertIsObject($obj->author);
		$this->assertEquals('Person', $obj->author->{'@type'});
		$this->assertEquals('John Doe', $obj->author->name);
	}

	public function testSingleOrganizationAuthor(): void {
		$article = new Article(
			headline: 'Editorial article',
			author: new Organization(name: 'Example Editorial Team'),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $article);
		$obj = json_decode($json);

		$this->assertIsObject($obj->author);
		$this->assertEquals('Organization', $obj->author->{'@type'});
		$this->assertEquals('Example Editorial Team', $obj->author->name);
	}
}
