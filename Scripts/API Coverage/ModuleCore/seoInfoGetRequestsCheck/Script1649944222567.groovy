import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable as GlobalVariable



WebUI.comment('TEST CASE: seoInfo. related endpoints check')


'CREATE A CATEGORY THAT CONTAINS SEO INFORMATION'
categoryCreate = WS.callTestCase(findTestCase('Test Cases/API Coverage/ModuleCatalog/categoryCreate'),
	null)
categoryUpdate = WS.callTestCase(findTestCase('Test Cases/API Coverage/ModuleCatalog/categoryUpdate'),
		null)


'VERIFY THE SEOINFO HAS BEEN ADDED'
seoInfoGet = WS.sendRequest(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Core/seoInfosGet',[
	('slugName') : GlobalVariable.semanticUrl
	]))
WS.verifyElementPropertyValue(seoInfoGet, '[0].objectId', GlobalVariable.categoryId, FailureHandling.CONTINUE_ON_FAILURE)


'DELETE CATEGORY'
categoryDelete = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Catalog/ListentriesDelete', [
	('objectIds') : GlobalVariable.categoryId
	]))

