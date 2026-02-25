<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Organization extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Organization';

	public function __construct(
		public string $name,
		public null|string $url = null,
		public null|string $logo = null,
		public null|string $description = null,
		public null|string $email = null,
		public null|string $telephone = null,
		public null|PostalAddress $address = null,
		public null|ContactPoint $contactPoint = null,
		/** @var string[] $sameAs */
		public null|array $sameAs = null,
		public null|string $foundingDate = null,
		public null|string $alternateName = null,
		public null|string $legalName = null,
		public null|QuantitativeValue $numberOfEmployees = null,
		public null|string $taxID = null,
		public null|string $vatID = null,
		public null|string $naics = null,
		public null|string $duns = null,
		public null|string $leiCode = null,
		public null|string $iso6523Code = null,
		public null|string $globalLocationNumber = null,
	) {}
}
