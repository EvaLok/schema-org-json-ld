<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\Enum\ItemAvailability;
use EvaLok\SchemaOrgJsonLd\v1\Enum\OfferItemCondition;
use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateOffer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateRating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Brand;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Certification;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Offer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PeopleAudience;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Product;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ProductGroup;
use EvaLok\SchemaOrgJsonLd\v1\Schema\QuantitativeValue;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Rating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Review;
use EvaLok\SchemaOrgJsonLd\v1\Schema\SizeSpecification;
use PHPUnit\Framework\TestCase;

final class ProductTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new Product(
			name: 'Executive Anvil',
			image: [
				'https://example.com/photos/1x1/photo.jpg',
				'https://example.com/photos/4x3/photo.jpg',
			],
			description: 'Sleeker than ACME\'s Classic Anvil.',
			sku: '0446310786',
			offers: [
				new Offer(
					url: 'https://example.com/anvil',
					priceCurrency: 'USD',
					price: 119.99,
					itemCondition: OfferItemCondition::NewCondition,
					availability: ItemAvailability::InStock,
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Product', $obj->{'@type'});
		$this->assertEquals('Executive Anvil', $obj->name);
		$this->assertEquals(
			[
				'https://example.com/photos/1x1/photo.jpg',
				'https://example.com/photos/4x3/photo.jpg',
			],
			$obj->image,
		);
		$this->assertEquals('Sleeker than ACME\'s Classic Anvil.', $obj->description);
		$this->assertEquals('0446310786', $obj->sku);
		$this->assertCount(1, $obj->offers);
		$this->assertEquals('Offer', $obj->offers[0]->{'@type'});
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new Product(
			name: 'Executive Anvil',
			image: ['https://example.com/photos/1x1/photo.jpg'],
			description: 'Sleeker than ACME\'s Classic Anvil.',
			sku: '0446310786',
			offers: [
				new Offer(
					url: 'https://example.com/anvil',
					priceCurrency: 'USD',
					price: 119.99,
					itemCondition: OfferItemCondition::NewCondition,
					availability: ItemAvailability::InStock,
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'brand'));
		$this->assertFalse(property_exists($obj, 'mpn'));
		$this->assertFalse(property_exists($obj, 'weight'));
		$this->assertFalse(property_exists($obj, 'aggregateRating'));
		$this->assertFalse(property_exists($obj, 'review'));
		$this->assertFalse(property_exists($obj, 'color'));
		$this->assertFalse(property_exists($obj, 'material'));
		$this->assertFalse(property_exists($obj, 'pattern'));
		$this->assertFalse(property_exists($obj, 'size'));
		$this->assertFalse(property_exists($obj, 'inProductGroupWithID'));
		$this->assertFalse(property_exists($obj, 'gtin'));
		$this->assertFalse(property_exists($obj, 'gtin8'));
		$this->assertFalse(property_exists($obj, 'gtin12'));
		$this->assertFalse(property_exists($obj, 'gtin13'));
		$this->assertFalse(property_exists($obj, 'gtin14'));
		$this->assertFalse(property_exists($obj, 'isbn'));
	}

	public function testOutputWithAdditionalTextProperties(): void {
		$schema = new Product(
			name: 'Executive Anvil',
			image: ['https://example.com/photos/1x1/photo.jpg'],
			description: 'Sleeker than ACME\'s Classic Anvil.',
			sku: '0446310786',
			offers: [],
			color: 'Black',
			material: 'Steel',
			pattern: 'Striped',
			size: 'Large',
			inProductGroupWithID: 'ANVIL-GROUP-1',
			gtin: '1234567890123',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Black', $obj->color);
		$this->assertEquals('Steel', $obj->material);
		$this->assertEquals('Striped', $obj->pattern);
		$this->assertEquals('Large', $obj->size);
		$this->assertEquals('ANVIL-GROUP-1', $obj->inProductGroupWithID);
		$this->assertEquals('1234567890123', $obj->gtin);
	}

	public function testOutputWithSizeSpecification(): void {
		$schema = new Product(
			name: 'Executive Anvil',
			image: ['https://example.com/photos/1x1/photo.jpg'],
			description: 'Sleeker than ACME\'s Classic Anvil.',
			sku: '0446310786',
			offers: [],
			size: new SizeSpecification(name: 'Large'),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('SizeSpecification', $obj->size->{'@type'});
		$this->assertEquals('Large', $obj->size->name);
	}

	public function testFullOutput(): void {
		$schema = new Product(
			name: 'Executive Anvil',
			image: ['https://example.com/photos/1x1/photo.jpg'],
			description: 'Sleeker than ACME\'s Classic Anvil.',
			sku: '0446310786',
			offers: [
				new Offer(
					url: 'https://example.com/anvil',
					priceCurrency: 'USD',
					price: 119.99,
					itemCondition: OfferItemCondition::NewCondition,
					availability: ItemAvailability::InStock,
				),
			],
			brand: new Brand(name: 'ACME'),
			mpn: 'ACME0444246625',
			weight: new QuantitativeValue(
				value: 55.67,
				unitCode: 'LBR',
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Brand', $obj->brand->{'@type'});
		$this->assertEquals('ACME', $obj->brand->name);
		$this->assertEquals('ACME0444246625', $obj->mpn);
		$this->assertEquals('QuantitativeValue', $obj->weight->{'@type'});
		$this->assertEquals(55.67, $obj->weight->value);
		$this->assertEquals('LBR', $obj->weight->unitCode);
	}

	public function testOutputWithAggregateRating(): void {
		$schema = new Product(
			name: 'Executive Anvil',
			image: ['https://example.com/photos/1x1/photo.jpg'],
			description: 'Sleeker than ACME\'s Classic Anvil.',
			sku: '0446310786',
			offers: [],
			aggregateRating: new AggregateRating(
				ratingValue: 4.4,
				reviewCount: 89,
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('AggregateRating', $obj->aggregateRating->{'@type'});
		$this->assertEquals(4.4, $obj->aggregateRating->ratingValue);
		$this->assertEquals(89, $obj->aggregateRating->reviewCount);
	}

	public function testOutputWithReview(): void {
		$schema = new Product(
			name: 'Executive Anvil',
			image: ['https://example.com/photos/1x1/photo.jpg'],
			description: 'Sleeker than ACME\'s Classic Anvil.',
			sku: '0446310786',
			offers: [],
			review: new Review(
				author: 'Jane Doe',
				reviewRating: new Rating(ratingValue: 5),
				reviewBody: 'Excellent anvil.',
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Review', $obj->review->{'@type'});
		$this->assertEquals('Jane Doe', $obj->review->author);
		$this->assertEquals('Rating', $obj->review->reviewRating->{'@type'});
		$this->assertEquals(5, $obj->review->reviewRating->ratingValue);
		$this->assertEquals('Excellent anvil.', $obj->review->reviewBody);
	}

	public function testOutputWithAggregateRatingAndReviewArray(): void {
		$schema = new Product(
			name: 'Executive Anvil',
			image: ['https://example.com/photos/1x1/photo.jpg'],
			description: 'Sleeker than ACME\'s Classic Anvil.',
			sku: '0446310786',
			offers: [],
			aggregateRating: new AggregateRating(ratingValue: 4.4),
			review: [
				new Review(
					author: 'Jane Doe',
					reviewRating: new Rating(ratingValue: 5),
				),
				new Review(
					author: 'John Doe',
					reviewRating: new Rating(ratingValue: 4),
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('AggregateRating', $obj->aggregateRating->{'@type'});
		$this->assertCount(2, $obj->review);
		$this->assertEquals('Review', $obj->review[0]->{'@type'});
		$this->assertEquals('Jane Doe', $obj->review[0]->author);
		$this->assertEquals('John Doe', $obj->review[1]->author);
	}

	public function testOutputWithAggregateOffer(): void {
		$schema = new Product(
			name: 'Executive Anvil',
			image: ['https://example.com/photos/1x1/photo.jpg'],
			description: 'Sleeker than ACME\'s Classic Anvil.',
			sku: '0446310786',
			offers: new AggregateOffer(
				lowPrice: 99.99,
				priceCurrency: 'USD',
				highPrice: 129.99,
				offerCount: 12,
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('AggregateOffer', $obj->offers->{'@type'});
		$this->assertEquals(99.99, $obj->offers->lowPrice);
		$this->assertEquals('USD', $obj->offers->priceCurrency);
		$this->assertEquals(129.99, $obj->offers->highPrice);
		$this->assertEquals(12, $obj->offers->offerCount);
	}

	public function testOutputWithMerchantListingProperties(): void {
		$schema = new Product(
			name: 'Executive Anvil Variant',
			image: ['https://example.com/photos/1x1/photo.jpg'],
			description: 'Variant product.',
			sku: '0446310786-VAR',
			offers: [],
			isVariantOf: new ProductGroup(name: 'Executive Anvil Family'),
			audience: new PeopleAudience(
				suggestedGender: 'Unisex',
				suggestedMinAge: 12,
			),
			hasCertification: [
				new Certification(
					name: 'EPREL',
					issuedBy: new Organization(name: 'EU Energy Labelling Authority'),
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('ProductGroup', $obj->isVariantOf->{'@type'});
		$this->assertEquals('Executive Anvil Family', $obj->isVariantOf->name);
		$this->assertEquals('PeopleAudience', $obj->audience->{'@type'});
		$this->assertEquals('Unisex', $obj->audience->suggestedGender);
		$this->assertEquals(12, $obj->audience->suggestedMinAge);
		$this->assertCount(1, $obj->hasCertification);
		$this->assertEquals('Certification', $obj->hasCertification[0]->{'@type'});
		$this->assertEquals('EPREL', $obj->hasCertification[0]->name);
	}
}
