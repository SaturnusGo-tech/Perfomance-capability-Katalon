package objects

import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import org.junit.Assert

import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObject

import com.kms.katalon.core.webui.common.WebUiCommonHelper
import com.kms.katalon.core.webui.exception.WebElementNotFoundException
import org.openqa.selenium.WebElement
import java.util.Random
import java.util.List

import org.openqa.selenium.WebDriver
import org.openqa.selenium.JavascriptExecutor
import com.kms.katalon.core.webui.driver.DriverFactory
import org.openqa.selenium.support.ui.WebDriverWait

import java.net.HttpURLConnection
import java.net.URL


/**
 * The CartPage class encapsulates actions and behaviors specific to the cart page.
 */
class CartPage {
	static final String CHECKOUT_PROCEED = "CHECKOUT_PROCEED"

	/**
	 * Clicks the 'Proceed to Checkout' button on the cart page.
	 */
	static void clickCheckoutProceedBtn() {
		// Wait for the 'Proceed to Checkout' button to be present
		boolean isButtonPresent = WebUI.waitForElementPresent(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/CartStage/' + CHECKOUT_PROCEED), 10)

		// Assert that the 'Proceed to Checkout' button is present
		Assert.assertTrue("The 'Proceed to Checkout' button is not present.", isButtonPresent)

		// Click the 'Proceed to Checkout' button
		WebUI.click(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/CartStage/' + CHECKOUT_PROCEED))

		// Additional verification or actions after the click can be added here
	}
	/**
	 * Checks the page load performance by verifying the page URL and HTTP status.
	 * @param pageUrl The URL of the page to be checked.
	 * @param timeout The timeout for page load in milliseconds.
	 */
	static void checkPageLoadPerformance(String pageUrl, int timeout) {
		// Check the HTTP status of the page
		HttpURLConnection connection = (HttpURLConnection) (new URL(pageUrl).openConnection())
		connection.setRequestMethod("GET")
		connection.connect()
		int responseCode = connection.getResponseCode()
		assert responseCode == 200 : "Expected HTTP status 200, but received " + responseCode

		// Measure the page load time
		WebDriver driver = DriverFactory.getWebDriver()
		long startTime = System.currentTimeMillis()
		driver.get(pageUrl)

		// Convert timeout from milliseconds to seconds for WebDriverWait
		int timeoutInSeconds = timeout / 1000
		new WebDriverWait(driver, timeoutInSeconds).until({ WebDriver d ->
			return ((JavascriptExecutor) d).executeScript("return document.readyState").equals("complete")
		} as com.google.common.base.Function<WebDriver, Boolean>)

		long endTime = System.currentTimeMillis()

		// Page load time in milliseconds
		long loadTime = endTime - startTime
		println("Page load time: " + loadTime + " ms")

		// Check if the page load time exceeds the specified timeout
		assert loadTime <= timeout : "Page load time exceeds " + timeout + " ms"
	}
}




