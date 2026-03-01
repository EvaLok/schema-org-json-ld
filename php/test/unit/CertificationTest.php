<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Certification;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Rating;
use PHPUnit\Framework\TestCase;

final class CertificationTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new Certification(
			name: 'EPREL',
			issuedBy: new Organization(name: 'EU Energy Labelling Authority'),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Certification', $obj->{'@type'});
		$this->assertEquals('EPREL', $obj->name);
		$this->assertEquals('Organization', $obj->issuedBy->{'@type'});
		$this->assertEquals('EU Energy Labelling Authority', $obj->issuedBy->name);
	}

	public function testFullOutput(): void {
		$schema = new Certification(
			name: 'Vehicle_CO2_Class',
			issuedBy: new Organization(name: 'National Transport Authority'),
			certificationIdentification: 'ABC-12345',
			certificationRating: new Rating(ratingValue: 4.5),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('ABC-12345', $obj->certificationIdentification);
		$this->assertEquals('Rating', $obj->certificationRating->{'@type'});
		$this->assertEquals(4.5, $obj->certificationRating->ratingValue);
	}
}
