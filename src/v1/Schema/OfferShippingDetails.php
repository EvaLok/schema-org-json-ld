<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class OfferShippingDetails extends TypedSchema {
	const A_SCHEMA_TYPE = 'OfferShippingDetails';

	function __construct(
		public DefinedRegion $shippingDestination,
		public null|MonetaryAmount $shippingRate = null,
		public null|ShippingDeliveryTime $deliveryTime = null,
		public null|bool $doesNotShip = null,
	){

	}
}
