<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Article;
use EvaLok\SchemaOrgJsonLd\v1\Schema\SpeakableSpecification;
use PHPUnit\Framework\TestCase;

final class SpeakableSpecificationTest extends TestCase {
	public function testCssSelectorStringSerializesCorrectly(): void {
		$speakable = new SpeakableSpecification(cssSelector: '.headline');
		$json = JsonLdGenerator::SchemaToJson(schema: $speakable);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('SpeakableSpecification', $obj->{'@type'});
		$this->assertEquals('.headline', $obj->cssSelector);
	}

	public function testXpathArraySerializesCorrectly(): void {
		$speakable = new SpeakableSpecification(
			xpath: [
				'/html/body/main/h1',
				'/html/body/main/article',
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $speakable);
		$obj = json_decode($json);

		$this->assertEquals([
			'/html/body/main/h1',
			'/html/body/main/article',
		], $obj->xpath);
	}

	public function testCssSelectorAndXpathSerializeCorrectly(): void {
		$speakable = new SpeakableSpecification(
			cssSelector: ['.headline', '.article-body'],
			xpath: '/html/body/main/article',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $speakable);
		$obj = json_decode($json);

		$this->assertEquals(['.headline', '.article-body'], $obj->cssSelector);
		$this->assertEquals('/html/body/main/article', $obj->xpath);
	}

	public function testNullPropertiesAreOmitted(): void {
		$speakable = new SpeakableSpecification();
		$json = JsonLdGenerator::SchemaToJson(schema: $speakable);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'cssSelector'));
		$this->assertFalse(property_exists($obj, 'xpath'));
	}

	public function testArticleWithSpeakableSerializesNestedSpeakableSpecification(): void {
		$article = new Article(
			headline: 'Example Article',
			speakable: new SpeakableSpecification(cssSelector: ['.headline', '.article-body']),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $article);
		$obj = json_decode($json);

		$this->assertEquals('Article', $obj->{'@type'});
		$this->assertEquals('SpeakableSpecification', $obj->speakable->{'@type'});
		$this->assertEquals(['.headline', '.article-body'], $obj->speakable->cssSelector);
	}

	public function testArticleWithoutSpeakableOmitsProperty(): void {
		$article = new Article(headline: 'Example Article');
		$json = JsonLdGenerator::SchemaToJson(schema: $article);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'speakable'));
	}
}
