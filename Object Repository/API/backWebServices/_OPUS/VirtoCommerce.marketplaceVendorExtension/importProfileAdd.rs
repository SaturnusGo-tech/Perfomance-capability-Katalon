<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>importProfileAdd</name>
   <tag></tag>
   <elementGuidId>17a9ea97-770c-40bc-ae9f-c25539fa2960</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;sellerId\&quot;: \&quot;${sellerId}\&quot;,\n  \&quot;sellerName\&quot;: \&quot;${sellerName}\&quot;,\n  \&quot;importProfile\&quot;: {\n    \&quot;name\&quot;: \&quot;${name}\&quot;,\n    \&quot;dataImporterType\&quot;: \&quot;${dataImporterType}\&quot;,\n    \&quot;settings\&quot;: [\n      {\n        \&quot;id\&quot;: null,\n        \&quot;restartRequired\&quot;: false,\n        \&quot;moduleId\&quot;: null,\n        \&quot;groupName\&quot;: \&quot;Import\&quot;,\n        \&quot;name\&quot;: \&quot;Import.Csv.Delimiter\&quot;,\n        \&quot;displayName\&quot;: null,\n        \&quot;isRequired\&quot;: true,\n        \&quot;isHidden\&quot;: false,\n        \&quot;valueType\&quot;: \&quot;ShortText\&quot;,\n        \&quot;allowedValues\&quot;: null,\n        \&quot;defaultValue\&quot;: \&quot;;\&quot;,\n        \&quot;isDictionary\&quot;: false,\n        \&quot;value\&quot;: \&quot;;\&quot;,\n        \&quot;values\&quot;: [\n          {\n            \&quot;id\&quot;: \&quot;;\&quot;,\n            \&quot;value\&quot;: \&quot;;\&quot;\n          }\n        ]\n      },\n      {\n        \&quot;id\&quot;: null,\n        \&quot;restartRequired\&quot;: false,\n        \&quot;moduleId\&quot;: null,\n        \&quot;groupName\&quot;: \&quot;Import\&quot;,\n        \&quot;name\&quot;: \&quot;Import.Csv.PageSize\&quot;,\n        \&quot;displayName\&quot;: null,\n        \&quot;isRequired\&quot;: false,\n        \&quot;isHidden\&quot;: false,\n        \&quot;valueType\&quot;: \&quot;PositiveInteger\&quot;,\n        \&quot;allowedValues\&quot;: null,\n        \&quot;defaultValue\&quot;: \&quot;50\&quot;,\n        \&quot;isDictionary\&quot;: false,\n        \&quot;value\&quot;: \&quot;50\&quot;,\n        \&quot;values\&quot;: [\n          {\n            \&quot;id\&quot;: \&quot;50\&quot;,\n            \&quot;value\&quot;: \&quot;50\&quot;\n          }\n        ]\n      },\n      {\n        \&quot;id\&quot;: null,\n        \&quot;restartRequired\&quot;: false,\n        \&quot;moduleId\&quot;: null,\n        \&quot;groupName\&quot;: \&quot;Import\&quot;,\n        \&quot;name\&quot;: \&quot;Vcmp.Import.Csv.CreateDictionaryValues\&quot;,\n        \&quot;displayName\&quot;: null,\n        \&quot;isRequired\&quot;: false,\n        \&quot;isHidden\&quot;: false,\n        \&quot;valueType\&quot;: \&quot;Boolean\&quot;,\n        \&quot;allowedValues\&quot;: null,\n        \&quot;defaultValue\&quot;: false,\n        \&quot;isDictionary\&quot;: false,\n        \&quot;values\&quot;: [{}]\n      }\n    ]\n  }\n}\n&quot;,
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
      <webElementGuid>96c47380-2fe5-460d-ade5-bfade71588f2</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.token}</value>
      <webElementGuid>01eae408-7219-4a90-9ccb-1e583494d6c6</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/api/vcmp/import/profiles</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'testProfile\n'</defaultValue>
      <description>created profile name</description>
      <id>9200ce9b-a8c1-41af-9170-1161a9026ace</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>'9706cb57-1c24-4127-a53e-55c3526a83b9'</defaultValue>
      <description>related vendor id</description>
      <id>0af6bbd6-5716-4072-9cfc-00d077322ae8</id>
      <masked>false</masked>
      <name>sellerId</name>
   </variables>
   <variables>
      <defaultValue>'test'</defaultValue>
      <description>name of the related vendor</description>
      <id>0a3f8d9f-a087-47c5-8630-59a7399e1909</id>
      <masked>false</masked>
      <name>sellerName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description>selected data importer type</description>
      <id>24fc066b-a361-4d26-bcd7-bcb77e6e5f42</id>
      <masked>false</masked>
      <name>dataImporterType</name>
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
