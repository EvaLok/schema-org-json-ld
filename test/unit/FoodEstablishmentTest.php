<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DayOfWeek;
use EvaLok\SchemaOrgJsonLd\v1\Schema\FoodEstablishment;
use EvaLok\SchemaOrgJsonLd\v1\Schema\GeoCoordinates;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OpeningHoursSpecification;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PostalAddress;
use PHPUnit\Framework\TestCase;

final class FoodEstablishmentTest extends TestCase {
	public function testMinimalOutput(): void {
		$foodEstablishment = new FoodEstablishment(
			name: 'Example Diner',
			address: new PostalAddress(streetAddress: '123 Main Street'),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $foodEstablishment);
		$obj = json_decode($json);

		$this->assertEquals('FoodEstablishment', $obj->{'@type'});
		$this->assertEquals('Example Diner', $obj->name);
	}

	public function testAcceptsReservations(): void {
		$foodEstablishment = new FoodEstablishment(
			name: 'Example Diner',
			address: new PostalAddress(streetAddress: '123 Main Street'),
			acceptsReservations: true,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $foodEstablishment);
		$obj = json_decode($json);

		$this->assertTrue($obj->acceptsReservations);
	}

	public function testAcceptsReservationsUrl(): void {
		$foodEstablishment = new FoodEstablishment(
			name: 'Example Diner',
			address: new PostalAddress(streetAddress: '123 Main Street'),
			acceptsReservations: 'https://example.com/reserve',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $foodEstablishment);
		$obj = json_decode($json);

		$this->assertEquals('https://example.com/reserve', $obj->acceptsReservations);
	}

	public function testInheritedProperties(): void {
		$foodEstablishment = new FoodEstablishment(
			name: 'Example Diner',
			address: new PostalAddress(streetAddress: '123 Main Street'),
			telephone: '+31-20-123-4567',
			geo: new GeoCoordinates(latitude: 52.37022, longitude: 4.89517),
			openingHoursSpecification: [
				new OpeningHoursSpecification(
					dayOfWeek: DayOfWeek::Monday,
					opens: '09:00',
					closes: '18:00',
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $foodEstablishment);
		$obj = json_decode($json);

		$this->assertEquals('+31-20-123-4567', $obj->telephone);
		$this->assertEquals('GeoCoordinates', $obj->geo->{'@type'});
		$this->assertEquals('OpeningHoursSpecification', $obj->openingHoursSpecification[0]->{'@type'});
	}
}
