<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\Enum\ItemAvailability;
use EvaLok\SchemaOrgJsonLd\v1\Enum\OfferItemCondition;
use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateRating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MobileApplication;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Offer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Rating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Review;
use PHPUnit\Framework\TestCase;

final class MobileApplicationTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new MobileApplication(
			name: 'Mobile App',
			offers: new Offer(
				url: 'https://example.com/mobile',
				priceCurrency: 'USD',
				price: 0,
				itemCondition: OfferItemCondition::NewCondition,
				availability: ItemAvailability::InStock,
			),
			aggregateRating: null,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('MobileApplication', $obj->{'@type'});
		$this->assertEquals('Mobile App', $obj->name);
		$this->assertEquals('Offer', $obj->offers->{'@type'});
	}

	public function testFullOutputWithInheritedProperties(): void {
		$schema = new MobileApplication(
			name: 'Mobile App',
			offers: [
				new Offer(
					url: 'https://example.com/mobile',
					priceCurrency: 'USD',
					price: 0,
					itemCondition: OfferItemCondition::NewCondition,
					availability: ItemAvailability::InStock,
				),
			],
			aggregateRating: new AggregateRating(ratingValue: 4.8, ratingCount: 1250),
			applicationCategory: 'BusinessApplication',
			operatingSystem: 'iOS 16+',
			review: new Review(
				author: 'Jane Doe',
				reviewRating: new Rating(ratingValue: 5),
				reviewBody: 'Excellent app!',
			),
			description: 'Example app description',
			screenshot: 'https://example.com/screenshot.png',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Offer', $obj->offers[0]->{'@type'});
		$this->assertEquals('AggregateRating', $obj->aggregateRating->{'@type'});
		$this->assertEquals(4.8, $obj->aggregateRating->ratingValue);
		$this->assertEquals('BusinessApplication', $obj->applicationCategory);
		$this->assertEquals('iOS 16+', $obj->operatingSystem);
		$this->assertEquals('Review', $obj->review->{'@type'});
		$this->assertEquals('Excellent app!', $obj->review->reviewBody);
		$this->assertEquals('Example app description', $obj->description);
		$this->assertEquals('https://example.com/screenshot.png', $obj->screenshot);
	}

	public function testFreeAppOfferWithZeroPrice(): void {
		$schema = new MobileApplication(
			name: 'Free Mobile App',
			offers: new Offer(
				url: 'https://example.com/free-mobile',
				priceCurrency: 'USD',
				price: 0,
				itemCondition: OfferItemCondition::NewCondition,
				availability: ItemAvailability::InStock,
			),
			aggregateRating: null,
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $schema));

		$this->assertEquals(0, $obj->offers->price);
		$this->assertEquals('USD', $obj->offers->priceCurrency);
	}

	public function testAggregateRatingAndNestedReview(): void {
		$schema = new MobileApplication(
			name: 'Rated Mobile App',
			offers: new Offer(
				url: 'https://example.com/mobile',
				priceCurrency: 'USD',
				price: 0,
				itemCondition: OfferItemCondition::NewCondition,
				availability: ItemAvailability::InStock,
			),
			aggregateRating: new AggregateRating(ratingValue: 4.9, ratingCount: 2500),
			review: new Review(
				author: 'Alex Reviewer',
				reviewRating: new Rating(ratingValue: 5),
				reviewBody: 'Great app experience.',
			),
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $schema));

		$this->assertEquals(4.9, $obj->aggregateRating->ratingValue);
		$this->assertEquals('Great app experience.', $obj->review->reviewBody);
		$this->assertEquals(5, $obj->review->reviewRating->ratingValue);
	}

	public function testApplicationCategoryAndOperatingSystem(): void {
		$schema = new MobileApplication(
			name: 'Business Mobile App',
			offers: new Offer(
				url: 'https://example.com/mobile',
				priceCurrency: 'USD',
				price: 0,
				itemCondition: OfferItemCondition::NewCondition,
				availability: ItemAvailability::InStock,
			),
			aggregateRating: null,
			applicationCategory: 'BusinessApplication',
			operatingSystem: 'iOS 17+',
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $schema));

		$this->assertEquals('BusinessApplication', $obj->applicationCategory);
		$this->assertEquals('iOS 17+', $obj->operatingSystem);
	}
}
