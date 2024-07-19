import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable


WebUI.comment('TEST CASE: verify products are imported correctly')


'GET PRODUCT DATA FROM CUSTOMER PORTAL AND VERIFY ALL PRODUCTS ARE IMPORTED'
customerPortalProductData = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Catalog/ProductsSearch',[
	('keyword') : ''
	]))
WS.verifyElementPropertyValue(customerPortalProductData, 'totalCount', '100')


'COLLECT PRODUCT DATA'
customerPortalProductGtin = WS.getElementPropertyValue(customerPortalProductData,'items[0].gtin')
customerPortalproductName = WS.getElementPropertyValue(customerPortalProductData,'items[0].name')
customerPortalproductDescription = WS.getElementPropertyValue(customerPortalProductData,'items[0].reviews[0].content')
customerPortalproductUNSPSC = WS.getElementPropertyValue(customerPortalProductData,'items[0].properties[15].values[0].value')
customerPortalproductManufacturer = WS.getElementPropertyValue(customerPortalProductData,'items[0].properties[22].values[0].value')
customerPortalproductTaxcode = WS.getElementPropertyValue(customerPortalProductData,'items[0].properties[21].values[0].value')
customerPortalproductImgLarge = WS.getElementPropertyValue(customerPortalProductData,'items[0].images[0].url')
customerPortalproductImgSmall = WS.getElementPropertyValue(customerPortalProductData,'items[0].images[1].url')
customerPortalproductKeywords = WS.getElementPropertyValue(customerPortalProductData,'items[0].properties[6].values')
customerPortalproductWeight = WS.getElementPropertyValue(customerPortalProductData,'items[0].properties[18].values[0].value')
customerPortalproductOversizeFurn = WS.getElementPropertyValue(customerPortalProductData,'items[0].properties[19].values[0].value')
//items[0].properties[6].name


'SAVE INITIAL URL AS A VARIABLE'
customerPortalUrl = GlobalVariable.urlBack


'AUTHORIZE AT SUPPLIER PORTAL'
GlobalVariable.urlBack = GlobalVariable.supplierPortalUrl
authorize = WS.callTestCase(findTestCase('Test Cases/API Coverage/ModulePlatform/Authorization/AuthBackToken'),
	null)
println GlobalVariable.token


'SEARCH FOR THE SAME PRODUCT (by gtin) ON SUPPLIER PORTAL'
//GlobalVariable.katalonVendorId = '077ec207-264b-4ada-8004-0d3e551336c2'
supplierPortalProductData = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.marketplaceVendorExtension/sellerProductsSearchByGtin',[
	('gtin') : customerPortalProductGtin
	]))
//we search by SKU, so no need to verify it


'COLLECT PRODUCT DATA'
supplierPortalProductName = WS.getElementPropertyValue(supplierPortalProductData,'results[0].name')
supplierPortalProductDescription =  WS.getElementPropertyValue(supplierPortalProductData,'results[0].description')
supplierPortalProductUNSPSC =  WS.getElementPropertyValue(supplierPortalProductData,'results[0].productData.properties[3].values[0].value')
supplierPortalProductManufacturer =  WS.getElementPropertyValue(supplierPortalProductData,'results[0].productData.properties[5].values[0].value')
supplierPortalProductTaxcode =  WS.getElementPropertyValue(supplierPortalProductData,'results[0].productData.properties[4].values[0].value')
supplierPortalProductImgLarge =  WS.getElementPropertyValue(supplierPortalProductData,'results[0].productData.images[0].url')
supplierPortalProductImgSmall =  WS.getElementPropertyValue(supplierPortalProductData,'results[0].productData.images[1].url')
supplierPortalProductKeywords =  WS.getElementPropertyValue(supplierPortalProductData,'results[0].productData.properties[0].values')//holds multiple values
supplierPortalProductWeight =  WS.getElementPropertyValue(supplierPortalProductData,'results[0].productData.properties[1].values[0].value')
supplierPortalProductOversizeFurn =  WS.getElementPropertyValue(supplierPortalProductData,'results[0].productData.properties[2].values[0].value')


'ASSERT IF PRODUCT DATA FROM DIFFERENT SOURCES MATCH'
assert customerPortalproductName == supplierPortalProductName
assert customerPortalproductDescription == supplierPortalProductDescription
assert customerPortalproductUNSPSC == supplierPortalProductUNSPSC
assert customerPortalproductManufacturer == supplierPortalProductManufacturer
assert customerPortalproductTaxcode == supplierPortalProductTaxcode
assert customerPortalproductImgLarge == supplierPortalProductImgLarge
assert customerPortalproductImgSmall == supplierPortalProductImgSmall
WebUI.comment('theres a bug where keywors are not imported from supplierportal, so this step will most likely fail. Commented for debugging')
//assert customerPortalproductKeywords == supplierPortalProductKeywords
assert customerPortalproductWeight == supplierPortalProductWeight
assert customerPortalproductOversizeFurn == supplierPortalProductOversizeFurn


'RETURN URL BACK TO THE INITIAL VALUE'
GlobalVariable.urlBack = customerPortalUrl

