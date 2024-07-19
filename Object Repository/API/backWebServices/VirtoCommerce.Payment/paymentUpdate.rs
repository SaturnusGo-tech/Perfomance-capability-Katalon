<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>paymentUpdate</name>
   <tag></tag>
   <elementGuidId>99a9f22a-e5fb-451d-9248-698ca24274af</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n            \&quot;paymentMethodType\&quot;: \&quot;Unknown\&quot;,\n            \&quot;paymentMethodGroupType\&quot;: \&quot;Manual\&quot;,\n            \&quot;code\&quot;: \&quot;PurchaseOrderPaymentMethod\&quot;,\n            \&quot;name\&quot;: \&quot;PurchaseOrderPaymentMethod\&quot;,\n            \&quot;logoUrl\&quot;: null,\n            \&quot;isActive\&quot;: false,\n            \&quot;priority\&quot;: 0,\n            \&quot;isAvailableForPartial\&quot;: false,\n            \&quot;currency\&quot;: null,\n            \&quot;price\&quot;: 0.0,\n            \&quot;priceWithTax\&quot;: 0.0,\n            \&quot;total\&quot;: 0.0,\n            \&quot;totalWithTax\&quot;: 0.0,\n            \&quot;discountAmount\&quot;: 0.0,\n            \&quot;discountAmountWithTax\&quot;: 0.0,\n            \&quot;storeId\&quot;: \&quot;opus\&quot;,\n            \&quot;description\&quot;: null,\n            \&quot;typeName\&quot;: \&quot;PurchaseOrderPaymentMethod\&quot;,\n            \&quot;settings\&quot;: [\n                {\n                    \&quot;itHasValues\&quot;: false,\n                    \&quot;objectId\&quot;: \&quot;4adc8a27-de0a-4aef-9172-e4cf99aa6b98\&quot;,\n                    \&quot;objectType\&quot;: \&quot;PurchaseOrderPaymentMethod\&quot;,\n                    \&quot;isReadOnly\&quot;: false,\n                    \&quot;value\&quot;: null,\n                    \&quot;id\&quot;: null,\n                    \&quot;restartRequired\&quot;: false,\n                    \&quot;moduleId\&quot;: \&quot;Opus\&quot;,\n                    \&quot;groupName\&quot;: \&quot;OPUS\&quot;,\n                    \&quot;name\&quot;: \&quot;Opus.NullSetting\&quot;,\n                    \&quot;displayName\&quot;: null,\n                    \&quot;isRequired\&quot;: false,\n                    \&quot;isHidden\&quot;: false,\n                    \&quot;valueType\&quot;: \&quot;Boolean\&quot;,\n                    \&quot;allowedValues\&quot;: null,\n                    \&quot;defaultValue\&quot;: false,\n                    \&quot;isDictionary\&quot;: false\n                }\n            ],\n            \&quot;taxType\&quot;: null,\n            \&quot;taxTotal\&quot;: 0.0,\n            \&quot;taxPercentRate\&quot;: 0.0,\n            \&quot;taxDetails\&quot;: null,\n            \&quot;id\&quot;: \&quot;4adc8a27-de0a-4aef-9172-e4cf99aa6b98\&quot;\n        }&quot;,
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
      <webElementGuid>1bd4eee8-d81a-4680-b26a-5480bf235c92</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>api_key</name>
      <type>Main</type>
      <value>${GlobalVariable.api_key}</value>
      <webElementGuid>652001c9-f625-4616-ab2e-3d6a5e33acc8</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/api/payment</restUrl>
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
      <id>365b4434-f262-4313-8679-fc3ea68d0bac</id>
      <masked>false</masked>
      <name>storeId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3ecee66a-4a1a-453a-973f-877c14fb50a6</id>
      <masked>false</masked>
      <name>keyword</name>
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
WS.verifyResponseStatusCode(response, 200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
