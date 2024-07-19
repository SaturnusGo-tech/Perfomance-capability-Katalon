import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.impl.HttpTextBodyContent
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

import groovy.json.JsonOutput
import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

'SET UP CONNECTION STRING JSON TEMPLATE'
String connectionStringJsonTemplate = '{\n    \"ConnectionString\": \"\",\n    \"AuthorizationToken\": \"\",\n    \"CustomHttpHeaders\": \"\"\n}'
connectionStringJsonTemplateParsed = new JsonSlurper().parseText(connectionStringJsonTemplate)


'IN THE PARSED TEMPLATE SET THE CONNECTION STRING VALUE'
connectionStringJsonTemplateParsed.ConnectionString = GlobalVariable.supplierConnectionString


'CONVERT CONNECTION STRING JSON TO ESCAPED JSON REPRESENTATION FORMAT'
connectionStringEscapedJson = JsonOutput.toJson(connectionStringJsonTemplateParsed)


'SET UP SUPPLIER UPDATE EQUEST FROM TEMPLATE'
supplierUpdateRequestTemplate = new File('TestDataFiles/supplierUpdate.json').text
supplierUpdateRequestTemplateParsed = new JsonSlurper().parseText(supplierUpdateRequestTemplate)


'ADD REQUIRED REQUEST VALUES AND CONNECTION STRING JSON'
supplierUpdateRequestTemplateParsed.supplierId = GlobalVariable.supplierId
supplierUpdateRequestTemplateParsed.supplierDetails.supplierId = GlobalVariable.supplierId
supplierUpdateRequestTemplateParsed.supplierDetails.Id = GlobalVariable.supplierId
supplierUpdateRequestTemplateParsed.supplierDetails.name = GlobalVariable.katalonVendorName
supplierUpdateRequestTemplateParsed.supplierDetails.outerId = GlobalVariable.supplierOuterId
supplierUpdateRequestTemplateParsed.supplierDetails.isConnected = true
supplierUpdateRequestTemplateParsed.supplierDetails.dataConnections[0].json = connectionStringEscapedJson


'BUILD REQUEST BODY JSON'
supplierUpdateRequestJson = new groovy.json.JsonBuilder(supplierUpdateRequestTemplateParsed).toString()


'SEND REQUEST TO UPDATE SUPPLIER'
RequestObject supplierUpdateObject = findTestObject('Object Repository/API/backWebServices/_OPUS/OPUS_mainExtensionModule/suppliersUpdate')
supplierUpdateObject.setBodyContent(new HttpTextBodyContent(supplierUpdateRequestJson))
supplierUpdate = WS.sendRequestAndVerify(supplierUpdateObject)


'LAUNCH INDEXATION'
indexRun = WS.callTestCase(findTestCase('Test Cases/API Coverage/ModuleSearch/BuildIndex'),null)


'VERIFY SUPPLIER HAS BEEN UPDATED'
supplierSearch = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/OPUS_mainExtensionModule/suppliersSearch',[
	('keyword') : GlobalVariable.katalonVendorName
	]))
WS.verifyElementPropertyValue(supplierSearch, 'results[0].name', GlobalVariable.katalonVendorName)
updatedConnectionString = WS.getElementPropertyValue(supplierSearch,'results[0].dataConnections[0].json')
expectedConnectionString = '{"ConnectionString":"https://omnia-vspat.paas.govirto.com/","AuthorizationToken":"","CustomHttpHeaders":""}'
assert updatedConnectionString == expectedConnectionString

