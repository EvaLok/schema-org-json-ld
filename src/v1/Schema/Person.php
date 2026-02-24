<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Person extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Person';

	public function __construct(
		public string $name,
		public null|string $url = null,
		public null|string $image = null,
		public null|string $email = null,
		public null|string $telephone = null,
		public null|string $jobTitle = null,
		public null|Organization $worksFor = null,
		/** @var string[] $sameAs */
		public null|array $sameAs = null,
		public null|string $description = null,
		public null|string $givenName = null,
		public null|string $familyName = null,
		public null|PostalAddress $address = null,
		public null|InteractionCounter $interactionStatistic = null,
		public null|InteractionCounter $agentInteractionStatistic = null,
		public null|string $identifier = null,
		public null|string $alternateName = null,
	) {}
}
