import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.impl.HttpTextBodyContent
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable


WebUI.comment('TEST CASE: import data to the platform')


'UPLOAD A FILE TO THE STORAGE'
fileUpload = WS.callTestCase(findTestCase('Test Cases/API Coverage/ModuleAssets/AssetFileUploadLocalStorageUpdated'),null)

/*
'GET AND COLLECT UPLOADED DATA'
importDataGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Platform/LoadMinifestGet',[
	('fileName') : GlobalVariable.fileName
	]))
List modulesList = WS.getElementPropertyValue(importDataGet,'modules.id')
modulesBlock = WS.getElementPropertyValue(importDataGet , 'modules')
platformVersion = WS.getElementPropertyValue(importDataGet, 'platformVersion') 


'GET AND PARSE IMPORT REQUEST TEMPLATE'
importRequestTemplate = new File('TestDataFiles/sampleDataImportRequestTemplate.json').text
importTemplateParsed = new JsonSlurper().parseText(importRequestTemplate)
importTemplateParsed.fileUrl = GlobalVariable.fileName
importTemplateParsed.fileName = GlobalVariable.fileName
importTemplateParsed.exportManifest.modules = modulesBlock
importTemplateParsed.exportManifest.platformVersion = platformVersion 
importTemplateParsed.modules = modulesList
importTemplateParsedJson = new groovy.json.JsonBuilder(importTemplateParsed).toString()
 

'IMPORT THE UPLOADED DATA'
RequestObject request = findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Platform/ImportSampleData')
request.setBodyContent(new HttpTextBodyContent(importTemplateParsedJson))
requestSend = WS.sendRequestAndVerify(request)

*/