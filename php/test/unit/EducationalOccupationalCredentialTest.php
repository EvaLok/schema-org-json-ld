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

	public function testFullOutput(): void {
		$schema = new EducationalOccupationalCredential(
			credentialCategory: 'professional certificate',
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('EducationalOccupationalCredential', $obj->{'@type'});
		$this->assertEquals('professional certificate', $obj->credentialCategory);
	}
}
