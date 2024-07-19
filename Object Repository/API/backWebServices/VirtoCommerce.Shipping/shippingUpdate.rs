<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>shippingUpdate</name>
   <tag></tag>
   <elementGuidId>319b1a30-507a-455b-9f97-b04e2c4e80f0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;code\&quot;: \&quot;string\&quot;,\n  \&quot;logoUrl\&quot;: \&quot;string\&quot;,\n  \&quot;isActive\&quot;: true,\n  \&quot;priority\&quot;: 0,\n  \&quot;taxType\&quot;: \&quot;string\&quot;,\n  \&quot;storeId\&quot;: \&quot;string\&quot;,\n  \&quot;settings\&quot;: [\n    {\n      \&quot;objectId\&quot;: \&quot;string\&quot;,\n      \&quot;objectType\&quot;: \&quot;string\&quot;,\n      \&quot;isReadOnly\&quot;: true,\n      \&quot;value\&quot;: {},\n      \&quot;id\&quot;: \&quot;string\&quot;,\n      \&quot;restartRequired\&quot;: true,\n      \&quot;moduleId\&quot;: \&quot;string\&quot;,\n      \&quot;groupName\&quot;: \&quot;string\&quot;,\n      \&quot;name\&quot;: \&quot;string\&quot;,\n      \&quot;displayName\&quot;: \&quot;string\&quot;,\n      \&quot;isRequired\&quot;: true,\n      \&quot;isHidden\&quot;: true,\n      \&quot;valueType\&quot;: \&quot;ShortText\&quot;,\n      \&quot;allowedValues\&quot;: [\n        {}\n      ],\n      \&quot;defaultValue\&quot;: {},\n      \&quot;isDictionary\&quot;: true\n    }\n  ],\n  \&quot;id\&quot;: \&quot;string\&quot;\n}&quot;,
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
   <restUrl>${GlobalVariable.urlBack}/api/shipping</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'opus'</defaultValue>
      <description></description>
      <id>365b4434-f262-4313-8679-fc3ea68d0bac</id>
      <masked>false</masked>
      <name>storeId</name>
   </variables>
   <variables>
      <defaultValue>'SupplierDefaultShippingMethod'</defaultValue>
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
