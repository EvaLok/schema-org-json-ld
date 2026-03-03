<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ItemList;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ListItem;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Product;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Rating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Review;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Thing;
use PHPUnit\Framework\TestCase;

final class ReviewTest extends TestCase {
	public function testMinimalOutput(): void {
		$review = new Review(
			author: 'Jane Doe',
			reviewRating: new Rating(ratingValue: 5),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $review);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Review', $obj->{'@type'});
		$this->assertEquals('Jane Doe', $obj->author);
		$this->assertEquals('Rating', $obj->reviewRating->{'@type'});
		$this->assertEquals(5, $obj->reviewRating->ratingValue);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$review = new Review(
			author: 'Jane Doe',
			reviewRating: new Rating(ratingValue: 5),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $review);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'reviewBody'));
		$this->assertFalse(property_exists($obj, 'datePublished'));
		$this->assertFalse(property_exists($obj, 'name'));
		$this->assertFalse(property_exists($obj, 'itemReviewed'));
		$this->assertFalse(property_exists($obj, 'positiveNotes'));
		$this->assertFalse(property_exists($obj, 'negativeNotes'));
	}

	public function testFullOutput(): void {
		$review = new Review(
			author: 'Jane Doe',
			reviewRating: new Rating(
				ratingValue: 4,
				bestRating: 5,
				worstRating: 1,
			),
			reviewBody: 'A great product!',
			datePublished: '2026-01-15',
			name: 'Excellent',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $review);
		$obj = json_decode($json);

		$this->assertEquals('Jane Doe', $obj->author);
		$this->assertEquals(4, $obj->reviewRating->ratingValue);
		$this->assertEquals(5, $obj->reviewRating->bestRating);
		$this->assertEquals(1, $obj->reviewRating->worstRating);
		$this->assertEquals('A great product!', $obj->reviewBody);
		$this->assertEquals('2026-01-15', $obj->datePublished);
		$this->assertEquals('Excellent', $obj->name);
	}

	public function testAuthorAsPerson(): void {
		$review = new Review(
			author: new Person(name: 'Jane Doe'),
			reviewRating: new Rating(ratingValue: 5),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $review);
		$obj = json_decode($json);

		$this->assertEquals('Person', $obj->author->{'@type'});
		$this->assertEquals('Jane Doe', $obj->author->name);
	}

	public function testAuthorAsOrganization(): void {
		$review = new Review(
			author: new Organization(name: 'Example Inc'),
			reviewRating: new Rating(ratingValue: 5),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $review);
		$obj = json_decode($json);

		$this->assertEquals('Organization', $obj->author->{'@type'});
		$this->assertEquals('Example Inc', $obj->author->name);
	}

	public function testOutputWithItemReviewedAsThing(): void {
		$review = new Review(
			author: 'Jane Doe',
			reviewRating: new Rating(ratingValue: 5),
			itemReviewed: new Thing(name: 'Executive Anvil'),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $review);
		$obj = json_decode($json);

		$this->assertEquals('Thing', $obj->itemReviewed->{'@type'});
		$this->assertEquals('Executive Anvil', $obj->itemReviewed->name);
	}

	public function testOutputWithItemReviewedAsProduct(): void {
		$review = new Review(
			author: 'Jane Doe',
			reviewRating: new Rating(ratingValue: 5),
			itemReviewed: new Product(
				name: 'Executive Anvil',
				image: ['https://example.com/photos/1x1/photo.jpg'],
				description: 'Sleeker than ACME\'s Classic Anvil.',
				sku: '0446310786',
				offers: [],
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $review);
		$obj = json_decode($json);

		$this->assertEquals('Product', $obj->itemReviewed->{'@type'});
		$this->assertEquals('Executive Anvil', $obj->itemReviewed->name);
	}

	public function testPositiveNotes(): void {
		$review = new Review(
			author: 'Jane Doe',
			reviewRating: new Rating(ratingValue: 5),
			positiveNotes: new ItemList(itemListElement: [
				new ListItem(position: 1, name: 'Consistent results'),
				new ListItem(position: 2, name: 'Still sharp after many uses'),
			]),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $review);
		$obj = json_decode($json);

		$this->assertEquals('ItemList', $obj->positiveNotes->{'@type'});
		$this->assertCount(2, $obj->positiveNotes->itemListElement);
		$this->assertEquals('ListItem', $obj->positiveNotes->itemListElement[0]->{'@type'});
		$this->assertEquals(1, $obj->positiveNotes->itemListElement[0]->position);
		$this->assertEquals('Consistent results', $obj->positiveNotes->itemListElement[0]->name);
		$this->assertEquals(2, $obj->positiveNotes->itemListElement[1]->position);
		$this->assertEquals('Still sharp after many uses', $obj->positiveNotes->itemListElement[1]->name);
	}

	public function testNegativeNotes(): void {
		$review = new Review(
			author: 'Jane Doe',
			reviewRating: new Rating(ratingValue: 5),
			negativeNotes: new ItemList(itemListElement: [
				new ListItem(position: 1, name: 'No child protection'),
			]),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $review);
		$obj = json_decode($json);

		$this->assertEquals('ItemList', $obj->negativeNotes->{'@type'});
		$this->assertCount(1, $obj->negativeNotes->itemListElement);
		$this->assertEquals('ListItem', $obj->negativeNotes->itemListElement[0]->{'@type'});
		$this->assertEquals(1, $obj->negativeNotes->itemListElement[0]->position);
		$this->assertEquals('No child protection', $obj->negativeNotes->itemListElement[0]->name);
	}

	public function testPositiveAndNegativeNotesTogether(): void {
		$review = new Review(
			author: 'Jane Doe',
			reviewRating: new Rating(ratingValue: 4),
			positiveNotes: new ItemList(itemListElement: [
				new ListItem(position: 1, name: 'Consistent results'),
				new ListItem(position: 2, name: 'Still sharp after many uses'),
			]),
			negativeNotes: new ItemList(itemListElement: [
				new ListItem(position: 1, name: 'No child protection'),
			]),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $review);
		$obj = json_decode($json);

		$this->assertEquals('ItemList', $obj->positiveNotes->{'@type'});
		$this->assertCount(2, $obj->positiveNotes->itemListElement);
		$this->assertEquals('ItemList', $obj->negativeNotes->{'@type'});
		$this->assertCount(1, $obj->negativeNotes->itemListElement);
	}
}
