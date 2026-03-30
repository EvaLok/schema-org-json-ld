<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\EducationalOccupationalCredential;
use PHPUnit\Framework\TestCase;

final class EducationalOccupationalCredentialTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new EducationalOccupationalCredential(
			credentialCategory: 'bachelor degree',
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('EducationalOccupationalCredential', $obj->{'@type'});
		$this->assertEquals('bachelor degree', $obj->credentialCategory);
	}

	public function testEmptyCredentialCategoryIsSerialized(): void {
		$schema = new EducationalOccupationalCredential(
			credentialCategory: '',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame('', $obj->credentialCategory);
	}

	public function testOnlyContextTypeAndCredentialCategoryAppear(): void {
		$schema = new EducationalOccupationalCredential(
			credentialCategory: 'industry certificate',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame(
			['@context', '@type', 'credentialCategory'],
			array_keys(get_object_vars($obj)),
		);
	}

	public function testExactCredentialCategoryValueRoundTrips(): void {
		$schema = new EducationalOccupationalCredential(
			credentialCategory: 'Level 3 - advanced certification',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame('Level 3 - advanced certification', $obj->credentialCategory);
	}
}
