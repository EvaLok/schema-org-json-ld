<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\BedDetails;
use PHPUnit\Framework\TestCase;

final class BedDetailsTest extends TestCase {
	public function testMinimalOutput(): void {
		$bedDetails = new BedDetails(numberOfBeds: 2);
		$json = JsonLdGenerator::SchemaToJson(schema: $bedDetails);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('BedDetails', $obj->{'@type'});
		$this->assertEquals(2, $obj->numberOfBeds);
		$this->assertFalse(property_exists($obj, 'typeOfBed'));
	}

	public function testOutputWithTypeOfBed(): void {
		$bedDetails = new BedDetails(
			numberOfBeds: 2,
			typeOfBed: 'Queen',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $bedDetails);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('BedDetails', $obj->{'@type'});
		$this->assertEquals(2, $obj->numberOfBeds);
		$this->assertEquals('Queen', $obj->typeOfBed);
	}

	public function testTypeOfBedOmittedWhenNull(): void {
		$bedDetails = new BedDetails(numberOfBeds: 1);
		$json = JsonLdGenerator::SchemaToJson(schema: $bedDetails);
		$obj = json_decode($json);

		$this->assertEquals(1, $obj->numberOfBeds);
		$this->assertFalse(property_exists($obj, 'typeOfBed'));
	}

	public function testZeroBedsIsSerialized(): void {
		$bedDetails = new BedDetails(numberOfBeds: 0);
		$json = JsonLdGenerator::SchemaToJson(schema: $bedDetails);
		$obj = json_decode($json);

		$this->assertSame(0, $obj->numberOfBeds);
	}

	public function testEmptyStringTypeOfBedIsSerialized(): void {
		$bedDetails = new BedDetails(
			numberOfBeds: 1,
			typeOfBed: '',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $bedDetails);
		$obj = json_decode($json);

		$this->assertSame('', $obj->typeOfBed);
	}
}
