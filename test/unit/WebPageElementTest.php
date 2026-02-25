<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Article;
use EvaLok\SchemaOrgJsonLd\v1\Schema\NewsArticle;
use EvaLok\SchemaOrgJsonLd\v1\Schema\WebPageElement;
use PHPUnit\Framework\TestCase;

final class WebPageElementTest extends TestCase {
	public function testWebPageElementSerializesWithRequiredFields(): void {
		$webPageElement = new WebPageElement(
			isAccessibleForFree: false,
			cssSelector: '.paywall',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $webPageElement);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('WebPageElement', $obj->{'@type'});
		$this->assertFalse($obj->isAccessibleForFree);
		$this->assertEquals('.paywall', $obj->cssSelector);
	}

	public function testArticleWithSinglePaywalledPartSerializesCorrectly(): void {
		$article = new NewsArticle(
			headline: 'Premium Article',
			isAccessibleForFree: false,
			hasPart: new WebPageElement(
				isAccessibleForFree: false,
				cssSelector: '.paywall',
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $article);
		$obj = json_decode($json);

		$this->assertEquals('NewsArticle', $obj->{'@type'});
		$this->assertFalse($obj->isAccessibleForFree);
		$this->assertEquals('WebPageElement', $obj->hasPart->{'@type'});
		$this->assertFalse($obj->hasPart->isAccessibleForFree);
		$this->assertEquals('.paywall', $obj->hasPart->cssSelector);
	}

	public function testArticleWithMultiplePaywalledPartsSerializesCorrectly(): void {
		$article = new Article(
			headline: 'Article with sections',
			isAccessibleForFree: false,
			hasPart: [
				new WebPageElement(isAccessibleForFree: false, cssSelector: '.section1'),
				new WebPageElement(isAccessibleForFree: false, cssSelector: '.section2'),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $article);
		$obj = json_decode($json);

		$this->assertEquals('Article', $obj->{'@type'});
		$this->assertFalse($obj->isAccessibleForFree);
		$this->assertCount(2, $obj->hasPart);
		$this->assertEquals('WebPageElement', $obj->hasPart[0]->{'@type'});
		$this->assertEquals('.section1', $obj->hasPart[0]->cssSelector);
		$this->assertEquals('WebPageElement', $obj->hasPart[1]->{'@type'});
		$this->assertEquals('.section2', $obj->hasPart[1]->cssSelector);
	}

	public function testArticleOmitsPaywalledPropertiesWhenNotSet(): void {
		$article = new Article(headline: 'Free article');
		$json = JsonLdGenerator::SchemaToJson(schema: $article);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'isAccessibleForFree'));
		$this->assertFalse(property_exists($obj, 'hasPart'));
	}
}
