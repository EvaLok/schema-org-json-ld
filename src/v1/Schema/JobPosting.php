<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class JobPosting extends TypedSchema {
	public const A_SCHEMA_TYPE = 'JobPosting';

	public function __construct(
		public string $title,
		public string $description,
		public string $datePosted,
		public Organization $hiringOrganization,
		public Place $jobLocation,
		public null|MonetaryAmount $baseSalary = null,
		public null|string $employmentType = null,
		public null|string $validThrough = null,
		public null|AdministrativeArea $applicantLocationRequirements = null,
		public null|string $jobLocationType = null,
		public null|bool $directApply = null,
	) {}
}
