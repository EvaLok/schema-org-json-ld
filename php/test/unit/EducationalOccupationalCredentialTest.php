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
}
