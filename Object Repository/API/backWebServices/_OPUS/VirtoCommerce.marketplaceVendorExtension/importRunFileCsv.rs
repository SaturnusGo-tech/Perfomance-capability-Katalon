<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>importRunFileCsv</name>
   <tag></tag>
   <elementGuidId>5e4b3500-bc35-4a01-bcf8-6ceedd8573aa</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;importProfile\&quot;: {\n\t\t\&quot;name\&quot;: \&quot;${name}\&quot;,\n\t\t\&quot;dataImporterType\&quot;: \&quot;${dataImporterType}\&quot;,\n\t\t\&quot;userId\&quot;: \&quot;${userId}\&quot;,\n\t\t\&quot;userName\&quot;: \&quot;${name}\&quot;,\n\t\t\&quot;settings\&quot;: [\n\t\t\t{\n\t\t\t\t\&quot;itHasValues\&quot;: true,\n\t\t\t\t\&quot;objectId\&quot;: \&quot;${objectId}\&quot;,\n\t\t\t\t\&quot;objectType\&quot;: \&quot;ImportProfile\&quot;,\n\t\t\t\t\&quot;isReadOnly\&quot;: false,\n\t\t\t\t\&quot;value\&quot;: \&quot;;\&quot;,\n\t\t\t\t\&quot;id\&quot;: \&quot;ed0f018d-8299-475c-8f2a-95d7966ce99a\&quot;,\n\t\t\t\t\&quot;restartRequired\&quot;: false,\n\t\t\t\t\&quot;moduleId\&quot;: null,\n\t\t\t\t\&quot;groupName\&quot;: \&quot;Import\&quot;,\n\t\t\t\t\&quot;name\&quot;: \&quot;Import.Csv.Delimiter\&quot;,\n\t\t\t\t\&quot;displayName\&quot;: null,\n\t\t\t\t\&quot;isRequired\&quot;: true,\n\t\t\t\t\&quot;isHidden\&quot;: false,\n\t\t\t\t\&quot;valueType\&quot;: \&quot;ShortText\&quot;,\n\t\t\t\t\&quot;defaultValue\&quot;: \&quot;;\&quot;,\n\t\t\t\t\&quot;isDictionary\&quot;: false\n\t\t\t},\n\t\t\t{\n\t\t\t\t\&quot;itHasValues\&quot;: true,\n\t\t\t\t\&quot;objectId\&quot;: \&quot;${objectId}\&quot;,\n\t\t\t\t\&quot;objectType\&quot;: \&quot;ImportProfile\&quot;,\n\t\t\t\t\&quot;isReadOnly\&quot;: false,\n\t\t\t\t\&quot;value\&quot;: 50,\n\t\t\t\t\&quot;id\&quot;: \&quot;023ec06c-39e3-4ca2-a725-732002bef477\&quot;,\n\t\t\t\t\&quot;restartRequired\&quot;: false,\n\t\t\t\t\&quot;moduleId\&quot;: null,\n\t\t\t\t\&quot;groupName\&quot;: \&quot;Import\&quot;,\n\t\t\t\t\&quot;name\&quot;: \&quot;Import.Csv.PageSize\&quot;,\n\t\t\t\t\&quot;displayName\&quot;: null,\n\t\t\t\t\&quot;isRequired\&quot;: false,\n\t\t\t\t\&quot;isHidden\&quot;: false,\n\t\t\t\t\&quot;valueType\&quot;: \&quot;PositiveInteger\&quot;,\n\t\t\t\t\&quot;defaultValue\&quot;: \&quot;50\&quot;,\n\t\t\t\t\&quot;isDictionary\&quot;: false\n\t\t\t},\n\t\t\t{\n\t\t\t\t\&quot;itHasValues\&quot;: false,\n\t\t\t\t\&quot;objectId\&quot;: \&quot;${objectId}\&quot;,\n\t\t\t\t\&quot;objectType\&quot;: \&quot;ImportProfile\&quot;,\n\t\t\t\t\&quot;isReadOnly\&quot;: false,\n\t\t\t\t\&quot;value\&quot;: null,\n\t\t\t\t\&quot;id\&quot;: null,\n\t\t\t\t\&quot;restartRequired\&quot;: false,\n\t\t\t\t\&quot;moduleId\&quot;: null,\n\t\t\t\t\&quot;groupName\&quot;: \&quot;VCMP|Import\&quot;,\n\t\t\t\t\&quot;name\&quot;: \&quot;Vcmp.Import.Csv.CreateDictionaryValues\&quot;,\n\t\t\t\t\&quot;displayName\&quot;: null,\n\t\t\t\t\&quot;isRequired\&quot;: false,\n\t\t\t\t\&quot;isHidden\&quot;: false,\n\t\t\t\t\&quot;valueType\&quot;: \&quot;Boolean\&quot;,\n\t\t\t\t\&quot;defaultValue\&quot;: false,\n\t\t\t\t\&quot;isDictionary\&quot;: false\n\t\t\t}\n\t\t],\n\t\t\&quot;typeName\&quot;: \&quot;ImportProfile\&quot;,\n\t\t\&quot;importFileUrl\&quot;: \&quot;${importFileUrl}\&quot;,\n\t\t\&quot;previewObjectCount\&quot;: 10,\n\t\t\&quot;createdDate\&quot;: \&quot;2023-03-17T00:15:31.234Z\&quot;,\n\t\t\&quot;modifiedDate\&quot;: \&quot;2023-03-17T00:15:31.234Z\&quot;,\n\t\t\&quot;createdBy\&quot;: \&quot;admin\&quot;,\n\t\t\&quot;modifiedBy\&quot;: \&quot;admin\&quot;,\n\t\t\&quot;id\&quot;: \&quot;${objectId}\&quot;\n\t}\n}&quot;,
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
   <restUrl>${GlobalVariable.urlBack}/api/vcmp/import/run</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'katalonProfile'</defaultValue>
      <description>name of the import profile</description>
      <id>522cebad-afc6-4df1-b50e-59154e7c8004</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>'b2faad3c-f644-4a33-921f-4e20f6ba682b'</defaultValue>
      <description>vendor Id (@vendors)</description>
      <id>152988d7-89d0-42c7-a686-f875ab5abf3f</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>'katalonVendor'</defaultValue>
      <description>name of the vendor of the related id</description>
      <id>b549deba-273e-43a4-aa7a-13fecf18181e</id>
      <masked>false</masked>
      <name>userName</name>
   </variables>
   <variables>
      <defaultValue>'148fe8d5-46cd-4a0b-920c-0d896c7362d7'</defaultValue>
      <description>id of the created import profile</description>
      <id>0b18d7e7-4567-4422-a109-c0a4c69bb220</id>
      <masked>false</masked>
      <name>objectId</name>
   </variables>
   <variables>
      <defaultValue>'https://omnia-vspat.paas.govirto.com/cms-content/assets/tmp/VSP_products.csv'</defaultValue>
      <description>uploaded file url</description>
      <id>36ce545f-af33-4f06-a99b-1e4668d88588</id>
      <masked>false</masked>
      <name>importFileUrl</name>
   </variables>
   <variables>
      <defaultValue>'CsvCdwProductImporter'</defaultValue>
      <description>selected data importer type</description>
      <id>1231f4b9-628c-4bb1-ab76-d9b13fd4c567</id>
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
