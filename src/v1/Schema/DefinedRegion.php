<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class DefinedRegion extends TypedSchema {
	public const A_SCHEMA_TYPE = 'DefinedRegion';

	/**
	 * @param string $addressCountry
	 * - eg: "US"
	 * @param string[] $addressRegion
	 * - eg: [ "CA", "NV", "AZ" ]
	 */
	public function __construct(
		public string $addressCountry,
		public array $addressRegion,
	) {}

}
