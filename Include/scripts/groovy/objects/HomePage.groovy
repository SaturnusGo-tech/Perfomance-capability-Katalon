package objects

import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import org.junit.Assert
import org.openqa.selenium.WebDriver
import org.openqa.selenium.JavascriptExecutor
import com.kms.katalon.core.webui.driver.DriverFactory
import org.openqa.selenium.support.ui.WebDriverWait

import java.net.HttpURLConnection
import java.net.URL

/**
 * The HomePage class encapsulates actions and behaviors specific to the home page.
 */
class HomePage {
	static final String EDUCATION_BTN = "EDUCATION_BTN"
	static final String CATEGORY_BTN = "CATEGORY_BTN"
	static final String CART_BTN = "CART_BTN"

	static final Map<String, String> altTexts = [
		Alt_Business_Products_Services: "Business Products & Services",
		Alt_Facilities: "Facilities",
		Alt_Information_Technology: "Information Technology",
		Alt_Food: "Food",
		Alt_Furniture: "Furniture",
		Alt_Furnishings: "Furnishings",
		Alt_Education: "Education",
		Alt_Construction: "Construction",
		Alt_Parks_Rec: "Parks & Rec",
		Alt_Disaster_Preparedness_Relief: "Disaster Preparedness & Relief",
		Alt_Public_Works: "Public Works",
		Alt_Public_Safety: "Public Safety"
	]

	/**
	 * Clicks the 'Education' button on the home page.
	 */
	static void clickEducationButton() {
		// Wait for the 'Education' button to be present
		boolean isButtonPresent = WebUI.waitForElementPresent(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/HomePageStage/' + EDUCATION_BTN), 10)

		// Assert that the 'Education' button is present
		Assert.assertTrue("The 'Education' button is not present.", isButtonPresent)

		// Click the 'Education' button
		WebUI.click(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/HomePageStage/' + EDUCATION_BTN))
	}

	/**
	 * Clicks the 'Category' button on the home page.
	 */
	static void clickCategoryButton() {
		// Wait for the 'Category' button to be present
		boolean isButtonPresent = WebUI.waitForElementPresent(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/HomePageStage/' + CATEGORY_BTN), 10)

		// Assert that the 'Category' button is present
		Assert.assertTrue("The 'Category' button is not present.", isButtonPresent)

		// Click the 'Category' button
		WebUI.click(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/HomePageStage/' + CATEGORY_BTN))
	}

	/**
	 * Clicks the 'Cart' button on the home page.
	 */
	static void clickCartButton() {
		// Wait for the 'Cart' button to be present
		boolean isButtonPresent = WebUI.waitForElementPresent(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/HomePageStage/' + CART_BTN), 10)

		// Assert that the 'Cart' button is present
		Assert.assertTrue("The 'Cart' button is not present.", isButtonPresent)

		// Click the 'Cart' button
		WebUI.click(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/HomePageStage/' + CART_BTN))
	}

	/**
	 * Checks for the presence of images by their alt texts on the home page.
	 */
	static void checkImagesByAltTexts() {
		altTexts.each { altKey, altValue ->
			try {
				// Define the test object for image with specific alt text
				TestObject imgTestObject = new TestObject().addProperty("xpath", ConditionType.EQUALS, "//img[@alt='" + altValue + "']", true)

				// Check if the image is present on the page
				boolean isImagePresent = WebUI.verifyElementPresent(imgTestObject, 5, FailureHandling.CONTINUE_ON_FAILURE)

				// Assert the presence of the image
				Assert.assertTrue("Image with alt text '" + altValue + "' is not present.", isImagePresent)

				// Log the presence of the image
				println("Image with alt text '" + altValue + "' is present.")
			} catch (Exception e) {
				// Log any errors encountered during the check
				println("Error checking image with alt text '" + altValue + "': " + e.message)
			}
		}
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