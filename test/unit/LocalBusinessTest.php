<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateRating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DayOfWeek;
use EvaLok\SchemaOrgJsonLd\v1\Schema\GeoCoordinates;
use EvaLok\SchemaOrgJsonLd\v1\Schema\LocalBusiness;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OpeningHoursSpecification;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PostalAddress;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Rating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Review;
use PHPUnit\Framework\TestCase;

final class LocalBusinessTest extends TestCase {
	public function testMinimalOutput(): void {
		$localBusiness = new LocalBusiness(
			name: 'Example Bistro',
			address: new PostalAddress(
				streetAddress: '123 Main Street',
				addressLocality: 'Amsterdam',
				postalCode: '1011AB',
				addressCountry: 'NL',
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $localBusiness);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('LocalBusiness', $obj->{'@type'});
		$this->assertEquals('Example Bistro', $obj->name);
		$this->assertEquals('PostalAddress', $obj->address->{'@type'});
		$this->assertEquals('123 Main Street', $obj->address->streetAddress);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$localBusiness = new LocalBusiness(
			name: 'Example Bistro',
			address: new PostalAddress(streetAddress: '123 Main Street'),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $localBusiness);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'url'));
		$this->assertFalse(property_exists($obj, 'telephone'));
		$this->assertFalse(property_exists($obj, 'description'));
		$this->assertFalse(property_exists($obj, 'image'));
		$this->assertFalse(property_exists($obj, 'priceRange'));
		$this->assertFalse(property_exists($obj, 'geo'));
		$this->assertFalse(property_exists($obj, 'openingHoursSpecification'));
		$this->assertFalse(property_exists($obj, 'aggregateRating'));
		$this->assertFalse(property_exists($obj, 'review'));
		$this->assertFalse(property_exists($obj, 'menu'));
		$this->assertFalse(property_exists($obj, 'servesCuisine'));
		$this->assertFalse(property_exists($obj, 'logo'));
		$this->assertFalse(property_exists($obj, 'email'));
		$this->assertFalse(property_exists($obj, 'sameAs'));
	}

	public function testFullOutputWithNestedTypesAndEnum(): void {
		$localBusiness = new LocalBusiness(
			name: 'Example Bistro',
			address: new PostalAddress(
				streetAddress: '123 Main Street',
				addressLocality: 'Amsterdam',
				addressRegion: 'NH',
				postalCode: '1011AB',
				addressCountry: 'NL',
			),
			url: 'https://example.com',
			telephone: '+31-20-123-4567',
			description: 'A cozy neighborhood restaurant.',
			image: [
				'https://example.com/images/front.jpg',
				'https://example.com/images/inside.jpg',
			],
			priceRange: '$$',
			geo: new GeoCoordinates(
				latitude: 52.37022,
				longitude: 4.89517,
			),
			openingHoursSpecification: [
				new OpeningHoursSpecification(
					dayOfWeek: DayOfWeek::Monday,
					opens: '09:00',
					closes: '18:00',
				),
			],
			aggregateRating: new AggregateRating(
				ratingValue: 4.6,
				ratingCount: 128,
			),
			review: [
				new Review(
					author: 'John Doe',
					reviewRating: new Rating(ratingValue: 5),
					reviewBody: 'Great food and service.',
				),
			],
			menu: 'https://example.com/menu',
			servesCuisine: 'Italian',
			logo: 'https://example.com/logo.png',
			email: 'hello@example.com',
			sameAs: [
				'https://www.facebook.com/example',
				'https://www.instagram.com/example',
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $localBusiness);
		$obj = json_decode($json);

		$this->assertEquals('https://example.com', $obj->url);
		$this->assertEquals('+31-20-123-4567', $obj->telephone);
		$this->assertEquals('A cozy neighborhood restaurant.', $obj->description);
		$this->assertEquals('https://example.com/images/front.jpg', $obj->image[0]);
		$this->assertEquals('$$', $obj->priceRange);
		$this->assertEquals('GeoCoordinates', $obj->geo->{'@type'});
		$this->assertEquals(52.37022, $obj->geo->latitude);
		$this->assertEquals('OpeningHoursSpecification', $obj->openingHoursSpecification[0]->{'@type'});
		$this->assertEquals('https://schema.org/Monday', $obj->openingHoursSpecification[0]->dayOfWeek);
		$this->assertEquals('AggregateRating', $obj->aggregateRating->{'@type'});
		$this->assertEquals(4.6, $obj->aggregateRating->ratingValue);
		$this->assertEquals('Review', $obj->review[0]->{'@type'});
		$this->assertEquals('John Doe', $obj->review[0]->author);
		$this->assertEquals('https://example.com/menu', $obj->menu);
		$this->assertEquals('Italian', $obj->servesCuisine);
		$this->assertEquals('https://example.com/logo.png', $obj->logo);
		$this->assertEquals('hello@example.com', $obj->email);
		$this->assertEquals('https://www.facebook.com/example', $obj->sameAs[0]);
	}
}
