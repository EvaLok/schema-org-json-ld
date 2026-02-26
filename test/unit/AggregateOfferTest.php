<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateOffer;
use PHPUnit\Framework\TestCase;

final class AggregateOfferTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new AggregateOffer(
			lowPrice: 99.99,
			priceCurrency: 'USD',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('AggregateOffer', $obj->{'@type'});
		$this->assertEquals(99.99, $obj->lowPrice);
		$this->assertEquals('USD', $obj->priceCurrency);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new AggregateOffer(
			lowPrice: 99.99,
			priceCurrency: 'USD',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertObjectNotHasProperty('highPrice', $obj);
		$this->assertObjectNotHasProperty('offerCount', $obj);
	}

	public function testFullOutput(): void {
		$schema = new AggregateOffer(
			lowPrice: 99.99,
			priceCurrency: 'USD',
			highPrice: 129.99,
			offerCount: 12,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('AggregateOffer', $obj->{'@type'});
		$this->assertEquals(99.99, $obj->lowPrice);
		$this->assertEquals('USD', $obj->priceCurrency);
		$this->assertEquals(129.99, $obj->highPrice);
		$this->assertEquals(12, $obj->offerCount);
	}
}
