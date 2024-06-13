<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class OfferShippingDetails extends TypedSchema {
	const A_SCHEMA_TYPE = 'OfferShippingDetails';

	function __construct(
		public DefinedRegion $shippingDestination,
		public MonetaryAmount $shippingRate,
		public ShippingDeliveryTime $deliveryTime,
		public null|bool $doesNotShip = null,
	){

	}
}
