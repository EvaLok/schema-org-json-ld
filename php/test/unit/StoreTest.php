<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Enum\DayOfWeek;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OpeningHoursSpecification;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PostalAddress;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Store;
use PHPUnit\Framework\TestCase;

final class StoreTest extends TestCase {
	public function testMinimalOutput(): void {
		$store = new Store(
			name: 'Example Store',
			address: new PostalAddress(streetAddress: '123 Main Street'),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $store);
		$obj = json_decode($json);

		$this->assertEquals('Store', $obj->{'@type'});
		$this->assertEquals('Example Store', $obj->name);
	}

	public function testInheritedProperties(): void {
		$store = new Store(
			name: 'Example Store',
			address: new PostalAddress(streetAddress: '123 Main Street'),
			telephone: '+31-20-123-4567',
			url: 'https://example.com',
			priceRange: '$$',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $store);
		$obj = json_decode($json);

		$this->assertEquals('Store', $obj->{'@type'});
		$this->assertEquals('+31-20-123-4567', $obj->telephone);
		$this->assertEquals('https://example.com', $obj->url);
		$this->assertEquals('$$', $obj->priceRange);
	}

	public function testOpeningHoursAcrossMultipleDays(): void {
		$store = new Store(
			name: 'Example Store',
			address: new PostalAddress(streetAddress: '123 Main Street'),
			openingHoursSpecification: [
				new OpeningHoursSpecification(
					dayOfWeek: DayOfWeek::Monday,
					opens: '09:00',
					closes: '18:00',
				),
				new OpeningHoursSpecification(
					dayOfWeek: DayOfWeek::Tuesday,
					opens: '09:00',
					closes: '18:00',
				),
			],
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $store));

		$this->assertCount(2, $obj->openingHoursSpecification);
		$this->assertEquals('https://schema.org/Monday', $obj->openingHoursSpecification[0]->dayOfWeek);
		$this->assertEquals('https://schema.org/Tuesday', $obj->openingHoursSpecification[1]->dayOfWeek);
	}

	public function testStoreTypeWithExtendedInheritedProperties(): void {
		$store = new Store(
			name: 'Example Store',
			address: new PostalAddress(streetAddress: '123 Main Street'),
			description: 'Neighborhood store',
			menu: 'https://example.com/cafe-menu',
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $store));

		$this->assertEquals('Store', $obj->{'@type'});
		$this->assertEquals('Neighborhood store', $obj->description);
		$this->assertEquals('https://example.com/cafe-menu', $obj->menu);
	}

	public function testStoreDepartmentAsArray(): void {
		$store = new Store(
			name: 'Main Store',
			address: new PostalAddress(streetAddress: '123 Main Street'),
			department: [
				new Store(
					name: 'Pharmacy',
					address: new PostalAddress(streetAddress: '123 Main Street - Unit B'),
				),
			],
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $store));

		$this->assertEquals('Store', $obj->department[0]->{'@type'});
		$this->assertEquals('Pharmacy', $obj->department[0]->name);
	}
}
