<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PostalAddress;
use PHPUnit\Framework\TestCase;

final class PersonTest extends TestCase {

	public function testMinimalOutput(): void {
		$person = new Person(name: 'John Doe');
		$json = JsonLdGenerator::SchemaToJson(schema: $person);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Person', $obj->{'@type'});
		$this->assertEquals('John Doe', $obj->name);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$person = new Person(name: 'John Doe');
		$json = JsonLdGenerator::SchemaToJson(schema: $person);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'url'));
		$this->assertFalse(property_exists($obj, 'image'));
		$this->assertFalse(property_exists($obj, 'email'));
		$this->assertFalse(property_exists($obj, 'telephone'));
		$this->assertFalse(property_exists($obj, 'jobTitle'));
		$this->assertFalse(property_exists($obj, 'worksFor'));
		$this->assertFalse(property_exists($obj, 'sameAs'));
		$this->assertFalse(property_exists($obj, 'description'));
		$this->assertFalse(property_exists($obj, 'givenName'));
		$this->assertFalse(property_exists($obj, 'familyName'));
		$this->assertFalse(property_exists($obj, 'address'));
	}

	public function testFullOutputWithNestedTypes(): void {
		$person = new Person(
			name: 'John Doe',
			url: 'https://example.com/john-doe',
			image: 'https://example.com/john-doe.jpg',
			email: 'john@example.com',
			telephone: '+1-555-123-4567',
			jobTitle: 'Editor',
			worksFor: new Organization(name: 'Example Inc.'),
			sameAs: [
				'https://www.linkedin.com/in/john-doe',
				'https://www.instagram.com/john-doe',
			],
			description: 'A profile description for John Doe.',
			givenName: 'John',
			familyName: 'Doe',
			address: new PostalAddress(
				streetAddress: '123 Main Street',
				addressLocality: 'Amsterdam',
				addressRegion: 'NH',
				postalCode: '1011AB',
				addressCountry: 'NL',
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $person);
		$obj = json_decode($json);

		$this->assertEquals('John Doe', $obj->name);
		$this->assertEquals('https://example.com/john-doe', $obj->url);
		$this->assertEquals('https://example.com/john-doe.jpg', $obj->image);
		$this->assertEquals('john@example.com', $obj->email);
		$this->assertEquals('+1-555-123-4567', $obj->telephone);
		$this->assertEquals('Editor', $obj->jobTitle);
		$this->assertEquals('Organization', $obj->worksFor->{'@type'});
		$this->assertEquals('Example Inc.', $obj->worksFor->name);
		$this->assertEquals('A profile description for John Doe.', $obj->description);
		$this->assertEquals('John', $obj->givenName);
		$this->assertEquals('Doe', $obj->familyName);
		$this->assertEquals('PostalAddress', $obj->address->{'@type'});
		$this->assertEquals('123 Main Street', $obj->address->streetAddress);
	}

	public function testSameAsSerializesAsArrayOfStrings(): void {
		$person = new Person(
			name: 'John Doe',
			sameAs: [
				'https://example.com/a',
				'https://example.com/b',
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $person);
		$obj = json_decode($json);

		$this->assertIsArray($obj->sameAs);
		$this->assertEquals('https://example.com/a', $obj->sameAs[0]);
		$this->assertEquals('https://example.com/b', $obj->sameAs[1]);
	}
}
