<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ShippingRateSettings;
use PHPUnit\Framework\TestCase;

final class ShippingRateSettingsTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new ShippingRateSettings();
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('ShippingRateSettings', $obj->{'@type'});
		$this->assertFalse(property_exists($obj, 'orderPercentage'));
		$this->assertFalse(property_exists($obj, 'weightPercentage'));
	}

	public function testFullOutput(): void {
		$schema = new ShippingRateSettings(
			orderPercentage: 0.1,
			weightPercentage: 0.2,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals(0.1, $obj->orderPercentage);
		$this->assertEquals(0.2, $obj->weightPercentage);
	}
}
