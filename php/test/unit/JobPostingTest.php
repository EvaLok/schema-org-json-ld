<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AdministrativeArea;
use EvaLok\SchemaOrgJsonLd\v1\Schema\EducationalOccupationalCredential;
use EvaLok\SchemaOrgJsonLd\v1\Schema\JobPosting;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MonetaryAmount;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OccupationalExperienceRequirements;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Place;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PostalAddress;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PropertyValue;
use PHPUnit\Framework\TestCase;

final class JobPostingTest extends TestCase {
	public function testMinimalOutput(): void {
		$jobPosting = new JobPosting(
			title: 'Senior Backend Engineer',
			description: '<p>Build scalable APIs.</p>',
			datePosted: '2026-02-24',
			hiringOrganization: new Organization(name: 'Example Corp'),
			jobLocation: new Place(
				name: 'Example HQ',
				address: new PostalAddress(
					streetAddress: '123 Main Street',
				),
			),
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $jobPosting);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('JobPosting', $obj->{'@type'});
		$this->assertEquals('Senior Backend Engineer', $obj->title);
		$this->assertEquals('<p>Build scalable APIs.</p>', $obj->description);
		$this->assertEquals('2026-02-24', $obj->datePosted);
		$this->assertEquals('Organization', $obj->hiringOrganization->{'@type'});
		$this->assertEquals('Place', $obj->jobLocation->{'@type'});
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$jobPosting = new JobPosting(
			title: 'Senior Backend Engineer',
			description: '<p>Build scalable APIs.</p>',
			datePosted: '2026-02-24',
			hiringOrganization: new Organization(name: 'Example Corp'),
			jobLocation: new Place(
				name: 'Example HQ',
				address: new PostalAddress(
					streetAddress: '123 Main Street',
				),
			),
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $jobPosting);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'baseSalary'));
		$this->assertFalse(property_exists($obj, 'employmentType'));
		$this->assertFalse(property_exists($obj, 'validThrough'));
		$this->assertFalse(property_exists($obj, 'applicantLocationRequirements'));
		$this->assertFalse(property_exists($obj, 'jobLocationType'));
		$this->assertFalse(property_exists($obj, 'directApply'));
		$this->assertFalse(property_exists($obj, 'identifier'));
		$this->assertFalse(property_exists($obj, 'educationRequirements'));
		$this->assertFalse(property_exists($obj, 'experienceRequirements'));
		$this->assertFalse(property_exists($obj, 'experienceInPlaceOfEducation'));
	}

	public function testFullOutputWithNestedSchemas(): void {
		$jobPosting = new JobPosting(
			title: 'Senior Backend Engineer',
			description: '<p>Build scalable APIs.</p>',
			datePosted: '2026-02-24',
			hiringOrganization: new Organization(
				name: 'Example Corp',
				url: 'https://example.com',
			),
			jobLocation: new Place(
				name: 'Example HQ',
				address: new PostalAddress(
					streetAddress: '123 Main Street',
					addressLocality: 'San Francisco',
					addressRegion: 'CA',
					postalCode: '94105',
					addressCountry: 'US',
				),
			),
			baseSalary: new MonetaryAmount(
				currency: 'USD',
				value: 120000.00,
			),
			employmentType: 'FULL_TIME',
			validThrough: '2026-04-30T23:59:59+00:00',
			applicantLocationRequirements: new AdministrativeArea(name: 'United States'),
			jobLocationType: 'TELECOMMUTE',
			directApply: true,
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $jobPosting);
		$obj = json_decode($json);

		$this->assertEquals('MonetaryAmount', $obj->baseSalary->{'@type'});
		$this->assertEquals(120000.00, $obj->baseSalary->value);
		$this->assertEquals('FULL_TIME', $obj->employmentType);
		$this->assertEquals('2026-04-30T23:59:59+00:00', $obj->validThrough);
		$this->assertEquals('AdministrativeArea', $obj->applicantLocationRequirements->{'@type'});
		$this->assertEquals('United States', $obj->applicantLocationRequirements->name);
		$this->assertEquals('TELECOMMUTE', $obj->jobLocationType);
		$this->assertTrue($obj->directApply);
	}

	public function testRemoteJobWithTelecommute(): void {
		$jobPosting = new JobPosting(
			title: 'Remote Backend Engineer',
			description: '<p>Work from anywhere in the US.</p>',
			datePosted: '2026-02-24',
			hiringOrganization: new Organization(name: 'Example Corp'),
			jobLocationType: 'TELECOMMUTE',
			applicantLocationRequirements: new AdministrativeArea(name: 'United States'),
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $jobPosting);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'jobLocation'));
		$this->assertEquals('TELECOMMUTE', $obj->jobLocationType);
		$this->assertEquals('United States', $obj->applicantLocationRequirements->name);
	}

	public function testIdentifierWithPropertyValue(): void {
		$jobPosting = new JobPosting(
			title: 'Senior Backend Engineer',
			description: '<p>Build scalable APIs.</p>',
			datePosted: '2026-02-24',
			hiringOrganization: new Organization(name: 'Example Corp'),
			identifier: new PropertyValue(
				name: 'MagsRUs',
				value: '1234567',
			),
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $jobPosting);
		$obj = json_decode($json);

		$this->assertEquals('PropertyValue', $obj->identifier->{'@type'});
		$this->assertEquals('MagsRUs', $obj->identifier->name);
		$this->assertEquals('1234567', $obj->identifier->value);
	}

	public function testEducationRequirementsWithCredentialObject(): void {
		$jobPosting = new JobPosting(
			title: 'Senior Backend Engineer',
			description: '<p>Build scalable APIs.</p>',
			datePosted: '2026-02-24',
			hiringOrganization: new Organization(name: 'Example Corp'),
			educationRequirements: new EducationalOccupationalCredential(
				credentialCategory: 'bachelor degree',
			),
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $jobPosting);
		$obj = json_decode($json);

		$this->assertEquals('EducationalOccupationalCredential', $obj->educationRequirements->{'@type'});
		$this->assertEquals('bachelor degree', $obj->educationRequirements->credentialCategory);
	}

	public function testEducationRequirementsWithString(): void {
		$jobPosting = new JobPosting(
			title: 'Senior Backend Engineer',
			description: '<p>Build scalable APIs.</p>',
			datePosted: '2026-02-24',
			hiringOrganization: new Organization(name: 'Example Corp'),
			educationRequirements: 'no requirements',
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $jobPosting);
		$obj = json_decode($json);

		$this->assertEquals('no requirements', $obj->educationRequirements);
	}

	public function testEducationRequirementsWithCredentialArray(): void {
		$jobPosting = new JobPosting(
			title: 'Senior Backend Engineer',
			description: '<p>Build scalable APIs.</p>',
			datePosted: '2026-02-24',
			hiringOrganization: new Organization(name: 'Example Corp'),
			educationRequirements: [
				new EducationalOccupationalCredential(credentialCategory: 'bachelor degree'),
				new EducationalOccupationalCredential(credentialCategory: 'professional certificate'),
			],
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $jobPosting);
		$obj = json_decode($json);

		$this->assertCount(2, $obj->educationRequirements);
		$this->assertEquals('EducationalOccupationalCredential', $obj->educationRequirements[0]->{'@type'});
		$this->assertEquals('bachelor degree', $obj->educationRequirements[0]->credentialCategory);
		$this->assertEquals('professional certificate', $obj->educationRequirements[1]->credentialCategory);
	}

	public function testExperienceRequirementsWithObject(): void {
		$jobPosting = new JobPosting(
			title: 'Senior Backend Engineer',
			description: '<p>Build scalable APIs.</p>',
			datePosted: '2026-02-24',
			hiringOrganization: new Organization(name: 'Example Corp'),
			experienceRequirements: new OccupationalExperienceRequirements(
				monthsOfExperience: 24,
			),
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $jobPosting);
		$obj = json_decode($json);

		$this->assertEquals('OccupationalExperienceRequirements', $obj->experienceRequirements->{'@type'});
		$this->assertEquals(24, $obj->experienceRequirements->monthsOfExperience);
	}

	public function testExperienceRequirementsWithString(): void {
		$jobPosting = new JobPosting(
			title: 'Senior Backend Engineer',
			description: '<p>Build scalable APIs.</p>',
			datePosted: '2026-02-24',
			hiringOrganization: new Organization(name: 'Example Corp'),
			experienceRequirements: 'no requirements',
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $jobPosting);
		$obj = json_decode($json);

		$this->assertEquals('no requirements', $obj->experienceRequirements);
	}

	public function testExperienceInPlaceOfEducationTrue(): void {
		$jobPosting = new JobPosting(
			title: 'Senior Backend Engineer',
			description: '<p>Build scalable APIs.</p>',
			datePosted: '2026-02-24',
			hiringOrganization: new Organization(name: 'Example Corp'),
			experienceInPlaceOfEducation: true,
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $jobPosting);
		$obj = json_decode($json);

		$this->assertTrue($obj->experienceInPlaceOfEducation);
	}
}
