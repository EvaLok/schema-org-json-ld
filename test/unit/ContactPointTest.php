<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ContactPoint;
use PHPUnit\Framework\TestCase;

final class ContactPointTest extends TestCase {

	public function testMinimalOutput(): void {
		$contactPoint = new ContactPoint();
		$json = JsonLdGenerator::SchemaToJson(schema: $contactPoint);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('ContactPoint', $obj->{'@type'});
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$contactPoint = new ContactPoint();
		$json = JsonLdGenerator::SchemaToJson(schema: $contactPoint);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'telephone'));
		$this->assertFalse(property_exists($obj, 'email'));
		$this->assertFalse(property_exists($obj, 'contactType'));
		$this->assertFalse(property_exists($obj, 'areaServed'));
		$this->assertFalse(property_exists($obj, 'availableLanguage'));
	}

	public function testFullOutput(): void {
		$contactPoint = new ContactPoint(
			telephone: '+1-800-555-1212',
			email: 'support@example.com',
			contactType: 'customer support',
			areaServed: 'US',
			availableLanguage: 'en',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $contactPoint);
		$obj = json_decode($json);

		$this->assertEquals('+1-800-555-1212', $obj->telephone);
		$this->assertEquals('support@example.com', $obj->email);
		$this->assertEquals('customer support', $obj->contactType);
		$this->assertEquals('US', $obj->areaServed);
		$this->assertEquals('en', $obj->availableLanguage);
	}
}
