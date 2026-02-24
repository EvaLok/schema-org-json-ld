<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class ImageObject extends TypedSchema {

	const A_SCHEMA_TYPE = 'ImageObject';

	public function __construct(
		public string $contentUrl,
		public null|string $url = null,
		public null|string $name = null,
		public null|string $caption = null,
		public null|string $description = null,
		public null|string $width = null,
		public null|string $height = null,
		public null|string $license = null,
		public null|string $acquireLicensePage = null,
		public null|string $creditText = null,
		public null|string $copyrightNotice = null,
		public null|Organization $creator = null,
		public null|string $datePublished = null,
		public null|string $uploadDate = null,
	) {

	}
}
