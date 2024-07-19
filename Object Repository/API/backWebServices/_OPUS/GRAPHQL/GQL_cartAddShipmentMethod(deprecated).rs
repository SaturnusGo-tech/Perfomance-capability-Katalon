<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GQL_cartAddShipmentMethod(deprecated)</name>
   <tag></tag>
   <elementGuidId>cf5ac510-8681-4884-8d73-2fff483de625</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;operationName\&quot;:\&quot;AddOrUpdateCartShipment\&quot;,\&quot;variables\&quot;:{\&quot;command\&quot;:{\&quot;storeId\&quot;:\&quot;${storeId}\&quot;,\&quot;userId\&quot;:\&quot;${userId}\&quot;,\&quot;cultureName\&quot;:\&quot;en-US\&quot;,\&quot;currencyCode\&quot;:\&quot;USD\&quot;,\&quot;cartId\&quot;:\&quot;${cartId}\&quot;,\&quot;shipment\&quot;:{\&quot;price\&quot;:${shipmentMethodAmount},\&quot;shipmentMethodCode\&quot;:\&quot;${shipmentMethodCode}\&quot;,\&quot;shipmentMethodOption\&quot;:\&quot;shipmentMethodOption\&quot;,\&quot;vendorId\&quot;:\&quot;${vendorId}\&quot;}}},\&quot;query\&quot;:\&quot;mutation AddOrUpdateCartShipment($command: InputAddOrUpdateCartShipmentType!) {\\n  addOrUpdateCartShipment(command: $command) {\\n    ...fullCartFields\\n  }\\n}\\n\\nfragment fullCartFields on CartType {\\n  id\\n  itemsQuantity\\n  purchaseOrderNumber\\n  comment\\n  availableGifts {\\n    id\\n    imageUrl\\n    name\\n    lineItemId\\n  }\\n  availableShippingMethods {\\n    id\\n    code\\n    logoUrl\\n    optionName\\n    optionDescription\\n    price {\\n      ...moneyFields\\n    }\\n    vendorId\\n  }\\n  availablePaymentMethods {\\n    code\\n    name\\n    description\\n    logoUrl\\n    paymentMethodGroupType\\n  }\\n  items {\\n    ...lineItemFields\\n  }\\n  gifts {\\n    id\\n    imageUrl\\n    name\\n    lineItemId\\n    quantity\\n  }\\n  coupons {\\n    code\\n    isAppliedSuccessfully\\n  }\\n  discounts {\\n    description\\n    amount\\n    coupon\\n  }\\n  shipments {\\n    id\\n    shipmentMethodCode\\n    shipmentMethodOption\\n    deliveryAddress {\\n      ...cartAddressFields\\n    }\\n    price {\\n      ...moneyFields\\n    }\\n    vendor {\\n      id\\n    }\\n    comment\\n  }\\n  payments {\\n    id\\n    paymentGatewayCode\\n    billingAddress {\\n      ...cartAddressFields\\n    }\\n    vendor {\\n      id\\n    }\\n    amount {\\n      amount\\n    }\\n  }\\n  currency {\\n    ...currencyFields\\n  }\\n  total {\\n    ...moneyFields\\n  }\\n  discountTotal {\\n    ...moneyFields\\n  }\\n  subTotal {\\n    ...moneyFields\\n  }\\n  shippingTotal {\\n    ...moneyFields\\n  }\\n  taxTotal {\\n    ...moneyFields\\n  }\\n  warnings {\\n    ...validationErrorFields\\n  }\\n  validationErrors(ruleSet: \\\&quot;items\\\&quot;) {\\n    ...validationErrorFields\\n  }\\n}\\n\\nfragment cartAddressFields on CartAddressType {\\n  id\\n  name\\n  organization\\n  firstName\\n  lastName\\n  line1\\n  line2\\n  city\\n  countryCode\\n  countryName\\n  regionId\\n  regionName\\n  postalCode\\n  phone\\n  email\\n  addressType\\n}\\n\\nfragment lineItemFields on LineItemType {\\n  id\\n  name\\n  sku\\n  quantity\\n  inStockQuantity\\n  imageUrl\\n  productId\\n  productType\\n  product {\\n    id\\n    slug\\n    minQuantity\\n    maxQuantity\\n    masterVariation {\\n      id\\n      slug\\n    }\\n    properties {\\n      ...propertyFields\\n    }\\n    keyProperties {\\n      ...propertyFields\\n    }\\n  }\\n  vendor {\\n    id\\n    name\\n    rating {\\n      value\\n      reviewCount\\n    }\\n    supplier {\\n      paymentMethods\\n      contractNumber\\n    }\\n  }\\n  extendedPrice {\\n    ...moneyFields\\n  }\\n  placedPrice {\\n    ...moneyFields\\n  }\\n  listPrice {\\n    ...moneyFields\\n  }\\n  salePrice {\\n    ...moneyFields\\n  }\\n  discountTotal {\\n    ...moneyFields\\n  }\\n  validationErrors {\\n    errorCode\\n    errorMessage\\n    errorParameters {\\n      key\\n      value\\n    }\\n  }\\n}\\n\\nfragment propertyFields on Property {\\n  name\\n  value\\n  type\\n  hidden\\n  valueType\\n  label\\n}\\n\\nfragment moneyFields on MoneyType {\\n  amount\\n  formattedAmount\\n  formattedAmountWithoutCurrency\\n  currency {\\n    ...currencyFields\\n  }\\n}\\n\\nfragment currencyFields on CurrencyType {\\n  code\\n  symbol\\n}\\n\\nfragment validationErrorFields on ValidationErrorType {\\n  errorCode\\n  errorMessage\\n  errorParameters {\\n    key\\n    value\\n  }\\n  objectId\\n  objectType\\n}\\n\&quot;}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>c0141233-e5b2-4c87-ad29-33a320387433</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.token}</value>
      <webElementGuid>776c4bc5-779d-4f0d-beb8-c96a5e92805e</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/graphql</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8fa03414-3103-4c91-82ad-96637eddcfc2</id>
      <masked>false</masked>
      <name>storeId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5da3be7c-8028-4edd-b41e-3f4b4077699a</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>de942695-b550-4318-8804-f824764577ff</id>
      <masked>false</masked>
      <name>cartId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>392d8206-d704-40b4-ab68-e81fa525160d</id>
      <masked>false</masked>
      <name>shipmentMethodAmount</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c4316f33-a55b-4206-b49b-3e93793b7ef5</id>
      <masked>false</masked>
      <name>shipmentMethodCode</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a60ad2f9-a233-4626-9c4f-1af1f75d35b4</id>
      <masked>false</masked>
      <name>shipmentMethodOption</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>0b85352f-c5b2-4656-bc3f-f24c1bf41b75</id>
      <masked>false</masked>
      <name>vendorId</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonOutput
import com.kms.katalon.core.util.KeywordUtil


ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
def prettyJson = JsonOutput.prettyPrint(response.getResponseBodyContent())
KeywordUtil.logInfo(prettyJson)
WS.verifyResponseStatusCode(response, 200)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
