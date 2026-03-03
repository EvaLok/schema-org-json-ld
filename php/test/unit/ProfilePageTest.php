<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ProfilePage;
use PHPUnit\Framework\TestCase;

final class ProfilePageTest extends TestCase {
	public function testMinimalOutputWithPersonMainEntity(): void {
		$schema = new ProfilePage(mainEntity: new Person(name: 'John Doe'));
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('ProfilePage', $obj->{'@type'});
		$this->assertEquals('Person', $obj->mainEntity->{'@type'});
		$this->assertEquals('John Doe', $obj->mainEntity->name);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new ProfilePage(mainEntity: new Person(name: 'John Doe'));
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertObjectNotHasProperty('dateCreated', $obj);
		$this->assertObjectNotHasProperty('dateModified', $obj);
	}

	public function testFullOutputWithOrganizationMainEntity(): void {
		$schema = new ProfilePage(
			mainEntity: new Organization(name: 'Acme Inc'),
			dateCreated: '2025-01-01',
			dateModified: '2025-01-31',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Organization', $obj->mainEntity->{'@type'});
		$this->assertEquals('Acme Inc', $obj->mainEntity->name);
		$this->assertEquals('2025-01-01', $obj->dateCreated);
		$this->assertEquals('2025-01-31', $obj->dateModified);
	}

	public function testMinimalOutputWithOrganizationMainEntity(): void {
		$schema = new ProfilePage(mainEntity: new Organization(name: 'Acme Inc'));
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $schema));

		$this->assertEquals('ProfilePage', $obj->{'@type'});
		$this->assertEquals('Organization', $obj->mainEntity->{'@type'});
		$this->assertEquals('Acme Inc', $obj->mainEntity->name);
	}

	public function testDateCreatedAndDateModifiedWithPersonMainEntity(): void {
		$schema = new ProfilePage(
			mainEntity: new Person(name: 'John Doe'),
			dateCreated: '2025-02-01',
			dateModified: '2025-02-15',
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $schema));

		$this->assertEquals('Person', $obj->mainEntity->{'@type'});
		$this->assertEquals('2025-02-01', $obj->dateCreated);
		$this->assertEquals('2025-02-15', $obj->dateModified);
	}
}
