<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Accommodation;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateRating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\BedDetails;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Brand;
use EvaLok\SchemaOrgJsonLd\v1\Schema\LocationFeatureSpecification;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PostalAddress;
use EvaLok\SchemaOrgJsonLd\v1\Schema\QuantitativeValue;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Rating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Review;
use EvaLok\SchemaOrgJsonLd\v1\Schema\VacationRental;
use PHPUnit\Framework\TestCase;

final class VacationRentalTest extends TestCase {
	public function testMinimalOutput(): void {
		$vacationRental = new VacationRental(
			name: 'Beach House',
			identifier: 'prop-123',
			image: ['https://example.com/photo1.jpg'],
			latitude: 42.12345,
			longitude: -101.12345,
			containsPlace: new Accommodation(
				occupancy: new QuantitativeValue(value: 6),
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $vacationRental);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('VacationRental', $obj->{'@type'});
		$this->assertEquals('Beach House', $obj->name);
		$this->assertEquals('prop-123', $obj->identifier);
		$this->assertEquals(42.12345, $obj->latitude);
		$this->assertEquals(-101.12345, $obj->longitude);
		$this->assertEquals('Accommodation', $obj->containsPlace->{'@type'});
		$this->assertEquals('QuantitativeValue', $obj->containsPlace->occupancy->{'@type'});
		$this->assertEquals(6, $obj->containsPlace->occupancy->value);
	}

	public function testFullOutput(): void {
		$vacationRental = new VacationRental(
			name: 'Beach House',
			identifier: 'prop-123',
			image: [
				'https://example.com/photo1.jpg',
				'https://example.com/photo2.jpg',
			],
			latitude: 42.12345,
			longitude: -101.12345,
			containsPlace: new Accommodation(
				occupancy: new QuantitativeValue(value: 6),
				additionalType: 'EntirePlace',
				numberOfBedrooms: 3,
				numberOfBathroomsTotal: 2,
				numberOfRooms: 5,
				floorSize: new QuantitativeValue(value: 85, unitCode: 'MTK'),
				bed: [
					new BedDetails(numberOfBeds: 2, typeOfBed: 'Queen'),
				],
				amenityFeature: [
					new LocationFeatureSpecification(name: 'wifi', value: true),
					new LocationFeatureSpecification(name: 'internetType', value: 'fiber'),
				],
			),
			additionalType: 'House',
			address: new PostalAddress(
				streetAddress: '123 Beach Road',
				addressLocality: 'Miami',
				addressRegion: 'FL',
				postalCode: '33101',
				addressCountry: 'US',
			),
			aggregateRating: new AggregateRating(ratingValue: 4.8, reviewCount: 120),
			brand: new Brand(name: 'Beach Stays'),
			checkinTime: '16:00',
			checkoutTime: '10:00',
			datePublished: '2026-02-26',
			description: 'Spacious beachfront vacation home.',
			knowsLanguage: ['en-US', 'es-ES'],
			review: [
				new Review(
					author: 'Jane Doe',
					reviewRating: new Rating(ratingValue: 5),
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $vacationRental);
		$obj = json_decode($json);

		$this->assertEquals('House', $obj->additionalType);
		$this->assertEquals('PostalAddress', $obj->address->{'@type'});
		$this->assertEquals('AggregateRating', $obj->aggregateRating->{'@type'});
		$this->assertEquals('Brand', $obj->brand->{'@type'});
		$this->assertEquals('16:00', $obj->checkinTime);
		$this->assertEquals('10:00', $obj->checkoutTime);
		$this->assertEquals('2026-02-26', $obj->datePublished);
		$this->assertEquals('Spacious beachfront vacation home.', $obj->description);
		$this->assertEquals('en-US', $obj->knowsLanguage[0]);
		$this->assertEquals('Review', $obj->review[0]->{'@type'});
		$this->assertEquals('BedDetails', $obj->containsPlace->bed[0]->{'@type'});
		$this->assertEquals('LocationFeatureSpecification', $obj->containsPlace->amenityFeature[0]->{'@type'});
		$this->assertTrue($obj->containsPlace->amenityFeature[0]->value);
		$this->assertEquals('fiber', $obj->containsPlace->amenityFeature[1]->value);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$vacationRental = new VacationRental(
			name: 'Beach House',
			identifier: 'prop-123',
			image: ['https://example.com/photo1.jpg'],
			latitude: 42.12345,
			longitude: -101.12345,
			containsPlace: new Accommodation(
				occupancy: new QuantitativeValue(value: 6),
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $vacationRental);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'additionalType'));
		$this->assertFalse(property_exists($obj, 'address'));
		$this->assertFalse(property_exists($obj, 'aggregateRating'));
		$this->assertFalse(property_exists($obj, 'brand'));
		$this->assertFalse(property_exists($obj, 'checkinTime'));
		$this->assertFalse(property_exists($obj, 'checkoutTime'));
		$this->assertFalse(property_exists($obj, 'datePublished'));
		$this->assertFalse(property_exists($obj, 'description'));
		$this->assertFalse(property_exists($obj, 'knowsLanguage'));
		$this->assertFalse(property_exists($obj, 'review'));
	}
}
