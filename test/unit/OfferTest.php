<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DefinedRegion;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ItemAvailability;
use EvaLok\SchemaOrgJsonLd\v1\Schema\QuantitativeValue;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Offer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OfferItemCondition;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OfferShippingDetails;
use EvaLok\SchemaOrgJsonLd\v1\Schema\UnitPriceSpecification;
use PHPUnit\Framework\TestCase;

final class OfferTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new Offer(
			url: 'https://example.com/anvil',
			priceCurrency: 'USD',
			price: 119.99,
			itemCondition: OfferItemCondition::NewCondition,
			availability: ItemAvailability::InStock,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Offer', $obj->{'@type'});
		$this->assertEquals('https://example.com/anvil', $obj->url);
		$this->assertEquals('USD', $obj->priceCurrency);
		$this->assertEquals(119.99, $obj->price);
		$this->assertEquals('https://schema.org/NewCondition', $obj->itemCondition);
		$this->assertEquals('https://schema.org/InStock', $obj->availability);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new Offer(
			url: 'https://example.com/anvil',
			priceCurrency: 'USD',
			price: 119.99,
			itemCondition: OfferItemCondition::NewCondition,
			availability: ItemAvailability::InStock,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'shippingDetails'));
		$this->assertFalse(property_exists($obj, 'validFrom'));
		$this->assertFalse(property_exists($obj, 'priceValidUntil'));
	}

	public function testWithShippingDetails(): void {
		$schema = new Offer(
			url: 'https://example.com/anvil',
			priceCurrency: 'USD',
			price: 119.99,
			itemCondition: OfferItemCondition::NewCondition,
			availability: ItemAvailability::InStock,
			shippingDetails: [
				new OfferShippingDetails(
					shippingDestination: new DefinedRegion(
						addressCountry: 'US',
						addressRegion: ['CA', 'NV', 'AZ'],
					),
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertCount(1, $obj->shippingDetails);
		$this->assertEquals('OfferShippingDetails', $obj->shippingDetails[0]->{'@type'});
		$this->assertEquals('DefinedRegion', $obj->shippingDetails[0]->shippingDestination->{'@type'});
		$this->assertEquals('US', $obj->shippingDetails[0]->shippingDestination->addressCountry);
		$this->assertEquals(['CA', 'NV', 'AZ'], $obj->shippingDetails[0]->shippingDestination->addressRegion);
	}

	public function testWithValidFrom(): void {
		$schema = new Offer(
			url: 'https://example.com/anvil',
			priceCurrency: 'USD',
			price: 119.99,
			itemCondition: OfferItemCondition::NewCondition,
			availability: ItemAvailability::InStock,
			validFrom: '2026-02-01T09:00:00+00:00',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('2026-02-01T09:00:00+00:00', $obj->validFrom);
	}

	public function testWithPriceValidUntil(): void {
		$schema = new Offer(
			url: 'https://example.com/anvil',
			priceCurrency: 'USD',
			price: 119.99,
			itemCondition: OfferItemCondition::NewCondition,
			availability: ItemAvailability::InStock,
			priceValidUntil: '2026-12-31',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('2026-12-31', $obj->priceValidUntil);
	}

	public function testItemConditionOmittedWhenNull(): void {
		$schema = new Offer(
			url: 'https://example.com/anvil',
			priceCurrency: 'USD',
			price: 119.99,
			availability: ItemAvailability::InStock,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'itemCondition'));
	}

	public function testWithPriceSpecification(): void {
		$schema = new Offer(
			url: 'https://example.com/anvil',
			priceCurrency: 'USD',
			price: 119.99,
			availability: ItemAvailability::InStock,
			priceSpecification: [
				new UnitPriceSpecification(
					price: 11.99,
					priceCurrency: 'USD',
					priceType: 'https://schema.org/StrikethroughPrice',
					referenceQuantity: new QuantitativeValue(
						value: 1,
						unitCode: 'KGM',
					),
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertCount(1, $obj->priceSpecification);
		$this->assertEquals('UnitPriceSpecification', $obj->priceSpecification[0]->{'@type'});
		$this->assertEquals(11.99, $obj->priceSpecification[0]->price);
		$this->assertEquals('USD', $obj->priceSpecification[0]->priceCurrency);
		$this->assertEquals('https://schema.org/StrikethroughPrice', $obj->priceSpecification[0]->priceType);
		$this->assertEquals('QuantitativeValue', $obj->priceSpecification[0]->referenceQuantity->{'@type'});
		$this->assertEquals(1, $obj->priceSpecification[0]->referenceQuantity->value);
		$this->assertEquals('KGM', $obj->priceSpecification[0]->referenceQuantity->unitCode);
	}
}
