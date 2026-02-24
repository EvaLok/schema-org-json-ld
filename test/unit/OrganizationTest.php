<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ContactPoint;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PostalAddress;
use PHPUnit\Framework\TestCase;

final class OrganizationTest extends TestCase {
	public function testMinimalOutput(): void {
		$organization = new Organization(name: 'Example Inc.');
		$json = JsonLdGenerator::SchemaToJson(schema: $organization);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Organization', $obj->{'@type'});
		$this->assertEquals('Example Inc.', $obj->name);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$organization = new Organization(name: 'Example Inc.');
		$json = JsonLdGenerator::SchemaToJson(schema: $organization);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'url'));
		$this->assertFalse(property_exists($obj, 'logo'));
		$this->assertFalse(property_exists($obj, 'description'));
		$this->assertFalse(property_exists($obj, 'email'));
		$this->assertFalse(property_exists($obj, 'telephone'));
		$this->assertFalse(property_exists($obj, 'address'));
		$this->assertFalse(property_exists($obj, 'contactPoint'));
		$this->assertFalse(property_exists($obj, 'sameAs'));
		$this->assertFalse(property_exists($obj, 'foundingDate'));
		$this->assertFalse(property_exists($obj, 'alternateName'));
		$this->assertFalse(property_exists($obj, 'legalName'));
	}

	public function testFullOutputWithNestedTypes(): void {
		$organization = new Organization(
			name: 'Example Inc.',
			url: 'https://example.com',
			logo: 'https://example.com/logo.png',
			description: 'An example organization.',
			email: 'hello@example.com',
			telephone: '+1-800-555-1212',
			address: new PostalAddress(
				streetAddress: '123 Main Street',
				addressLocality: 'Amsterdam',
				addressRegion: 'NH',
				postalCode: '1011AB',
				addressCountry: 'NL',
			),
			contactPoint: new ContactPoint(
				telephone: '+1-800-555-1212',
				email: 'support@example.com',
				contactType: 'customer support',
				areaServed: 'NL',
				availableLanguage: 'en',
			),
			sameAs: [
				'https://www.facebook.com/example',
				'https://www.linkedin.com/company/example',
			],
			foundingDate: '2020-01-01',
			alternateName: 'Example',
			legalName: 'Example Incorporated',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $organization);
		$obj = json_decode($json);

		$this->assertEquals('Example Inc.', $obj->name);
		$this->assertEquals('https://example.com', $obj->url);
		$this->assertEquals('https://example.com/logo.png', $obj->logo);
		$this->assertEquals('An example organization.', $obj->description);
		$this->assertEquals('hello@example.com', $obj->email);
		$this->assertEquals('+1-800-555-1212', $obj->telephone);
		$this->assertEquals('PostalAddress', $obj->address->{'@type'});
		$this->assertEquals('123 Main Street', $obj->address->streetAddress);
		$this->assertEquals('ContactPoint', $obj->contactPoint->{'@type'});
		$this->assertEquals('customer support', $obj->contactPoint->contactType);
		$this->assertEquals('https://www.facebook.com/example', $obj->sameAs[0]);
		$this->assertEquals('https://www.linkedin.com/company/example', $obj->sameAs[1]);
		$this->assertEquals('2020-01-01', $obj->foundingDate);
		$this->assertEquals('Example', $obj->alternateName);
		$this->assertEquals('Example Incorporated', $obj->legalName);
	}
}
