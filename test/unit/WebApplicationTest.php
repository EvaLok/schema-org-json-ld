<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\Enum\ItemAvailability;
use EvaLok\SchemaOrgJsonLd\v1\Enum\OfferItemCondition;
use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateRating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Offer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Rating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Review;
use EvaLok\SchemaOrgJsonLd\v1\Schema\WebApplication;
use PHPUnit\Framework\TestCase;

final class WebApplicationTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new WebApplication(
			name: 'Web App',
			offers: new Offer(
				url: 'https://example.com/web',
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
		$this->assertEquals('WebApplication', $obj->{'@type'});
		$this->assertEquals('Web App', $obj->name);
		$this->assertEquals('Offer', $obj->offers->{'@type'});
	}

	public function testFullOutputWithInheritedProperties(): void {
		$schema = new WebApplication(
			name: 'Web App',
			offers: [
				new Offer(
					url: 'https://example.com/web',
					priceCurrency: 'USD',
					price: 0,
					itemCondition: OfferItemCondition::NewCondition,
					availability: ItemAvailability::InStock,
				),
			],
			aggregateRating: new AggregateRating(ratingValue: 4.8, ratingCount: 1250),
			applicationCategory: 'BusinessApplication',
			operatingSystem: 'Web',
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
		$this->assertEquals('Web', $obj->operatingSystem);
		$this->assertEquals('Review', $obj->review->{'@type'});
		$this->assertEquals('Excellent app!', $obj->review->reviewBody);
		$this->assertEquals('Example app description', $obj->description);
		$this->assertEquals('https://example.com/screenshot.png', $obj->screenshot);
	}
}
