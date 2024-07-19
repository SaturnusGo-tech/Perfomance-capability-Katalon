import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable


WebUI.comment('TEST CASE: vendor portal. Products upload and import.')

/*
'UPLOAD FILE TO /tmp folder'
GlobalVariable.uploadedFileName = 'VSP_products.csv'
GlobalVariable.folderUrl = '/tmp'
//this request needs to be separated and added via callTestCase to
//be used in multiple case. Task for refactoring
uploadFileUrlLocal = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.Assets/vspProducts_AssetFileUploadLocal', [
	('folderUrl') : GlobalVariable.folderUrl,
	('fileName') : GlobalVariable.uploadedFileName
	]))
GlobalVariable.uploadFileUrl = WS.getElementPropertyValue(uploadFileUrlLocal, '[0].url')
*/

'GET VENDOR TOKEN TO AUTHORIZE THE REQUEST'
def login = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Platform/Authorization/AuthorizationTokenNew',[
	('username') : GlobalVariable.katalonVendorEmail,
	('password') : GlobalVariable.katalonVendorPassword
	]))
GlobalVariable.token = WS.getElementPropertyValue(login, 'token_type') + ' ' + WS.getElementPropertyValue(login, 'access_token')


'GET IMPORT RUN HISTORY'
importExecutionHistory = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.marketplaceVendorExtension/importProfileSearchExecutionHistory',[
	('profileId') : GlobalVariable.katalonImportProfileId
	]))
executionssInitialCount = WS.getElementPropertyValue(importExecutionHistory,'totalCount')


'IMPORT CSV FROM BLOB'
GlobalVariable.uploadFileUrl = GlobalVariable.OPUS_productsCsvUrl
//for some reason can only be authorized via token, no  apiKey auth. Bug mb?
fileImport = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.marketplaceVendorExtension/importRunFileCsv',[
	('name') : GlobalVariable.katalonImportProfileName,
	('userId') : GlobalVariable.katalonVendorId,
	('userName') : GlobalVariable.katalonVendorName,
	('objectId') : GlobalVariable.katalonImportProfileId,
	('importFileUrl') : GlobalVariable.uploadFileUrl,
	('dataImporterType') : GlobalVariable.dataImporterType
	]))
WebUI.comment(GlobalVariable.katalonImportProfileId)
WebUI.comment(GlobalVariable.uploadFileUrl)
WebUI.comment(GlobalVariable.katalonVendorId)
WebUI.comment(GlobalVariable.dataImporterType)
WebUI.comment(GlobalVariable.katalonVendorName)


'WAIT FOR IMPORT TO BE REGISTERED AS A SYSTEM EVENT'
def executionStatus
executionsUpdatedCount = executionssInitialCount
while (executionsUpdatedCount == executionssInitialCount) {
	importExecutionHistory = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.marketplaceVendorExtension/importProfileSearchExecutionHistory',[
		('profileId') : GlobalVariable.katalonImportProfileId
		]))
	executionsUpdatedCount = WS.getElementPropertyValue(importExecutionHistory,'totalCount')
	println executionssInitialCount
	println executionsUpdatedCount
	}

	
'VERIFY IMPORT OPERATION HAS BEEN FINISHED'
def importExecutionStatus
while (importExecutionStatus == null) {
	  importExecutionHistory = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.marketplaceVendorExtension/importProfileSearchExecutionHistory',[
		('profileId') : GlobalVariable.katalonImportProfileId
		]))
	  importExecutionStatus = WS.getElementPropertyValue(importExecutionHistory,'results[0].finished')
	  errorsCount = WS.getElementPropertyValue(importExecutionHistory,'results[0].errorsCount')
	  errors = WS.getElementPropertyValue(importExecutionHistory,'results[0].errors')
	  assert 	errorsCount == 0
	  WS.verifyElementPropertyValue(importExecutionHistory,'results[0].typeName', 'ImportRunHistory')
	  println errorsCount
	  println errors
	  println importExecutionStatus
	  }


'AUTHORIZE AS ADMIN'
WS.callTestCase(findTestCase('Test Cases/API Coverage/ModulePlatform/Authorization/AuthBackToken'),
	null)


'VERIFY PRODUCTS ARE IMPORTED'
WS.callTestCase(findTestCase('Test Cases/API Coverage/ModuleSearch/BuildIndex'),null)
adminTokenGet = WS.callTestCase(findTestCase('Test Cases/API Coverage/ModulePlatform/Authorization/AuthBackToken'),null)
importVerify = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.marketplaceVendorExtension/sellerProductsSearch',[
	('sellerId') : GlobalVariable.katalonVendorId
	]))
productsCount = WS.getElementPropertyValue(importVerify ,'totalCount')
assert productsCount == 100
WS.callTestCase(findTestCase('Test Cases/API Coverage/ModuleSearch/BuildIndex'),null)

