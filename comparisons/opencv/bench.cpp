#include <algorithm>
#include <chrono>
#include <cmath>
#include <iostream>
#include <opencv2/opencv.hpp>
#include <opencv2/features.hpp>
#include <string>
#include <utility>
#include <vector>

using Clock = std::chrono::steady_clock;

template <class Fn>
double time_once(Fn& fn) {
    auto t0 = Clock::now();
    fn();
    auto t1 = Clock::now();
    return std::chrono::duration<double, std::milli>(t1 - t0).count();
}

// Same rule as the Rust side: at least `iters` samples and at least 200 ms in total, capped at 5000.
template <class Fn>
double bench_ms(int warmup, int iters, Fn fn) {
    double first = 0.0;
    for (int i = 0; i < warmup; ++i) first = std::max(first, time_once(fn));
    int wanted = std::clamp(static_cast<int>(200.0 / std::max(first, 1e-6)), iters, 5000);
    std::vector<double> samples;
    samples.reserve(static_cast<size_t>(wanted));
    for (int i = 0; i < wanted; ++i) samples.push_back(time_once(fn));
    std::sort(samples.begin(), samples.end());
    return samples[samples.size() / 2];
}

struct Row {
    std::string id;
    double ms;
};

struct Digest {
    std::string id;
    int w, h;
    double mean;
};

static std::string json_escape(const std::string& in) {
    std::string out;
    for (char c : in) {
        if (c == '"' || c == '\\') out += '\\';
        out += c;
    }
    return out;
}

// HAL, SIMD and threading lines from cv::getBuildInformation(), recorded next to the timings.
static std::vector<std::string> build_lines() {
    static const char* keys[] = {"Parallel framework", "Custom HAL", "Baseline", "Dispatched code", "Intel IPP", "C++ flags (Release)", "Link-Time Optimization"};
    std::vector<std::string> out;
    std::string info = cv::getBuildInformation();
    size_t start = 0;
    while (start < info.size()) {
        size_t end = info.find('\n', start);
        if (end == std::string::npos) end = info.size();
        std::string line = info.substr(start, end - start);
        for (const char* key : keys) {
            if (line.find(key) != std::string::npos) {
                size_t first = line.find_first_not_of(' ');
                out.push_back(line.substr(first == std::string::npos ? 0 : first));
                break;
            }
        }
        start = end + 1;
    }
    return out;
}

static cv::Mat asset(const std::string& dir, const std::string& name, int flags) {
    cv::Mat m = cv::imread(dir + "/" + name, flags);
    if (m.empty()) {
        std::cerr << "Failed to load " << name << "\n";
        std::exit(1);
    }
    return m;
}

static double opencv_psnr(const cv::Mat& a, const cv::Mat& b) {
    cv::Mat s1;
    cv::absdiff(a, b, s1);
    s1.convertTo(s1, CV_32F);
    s1 = s1.mul(s1);
    cv::Scalar s = cv::sum(s1);
    double sse = s[0] + s[1] + s[2];
    if (sse <= 1e-10) return 100.0;
    double mse = sse / static_cast<double>(a.channels() * a.total());
    return 10.0 * std::log10((255.0 * 255.0) / mse);
}

static double opencv_ssim(const cv::Mat& i1, const cv::Mat& i2) {
    const double C1 = 6.5025, C2 = 58.5225;
    cv::Mat I1, I2;
    i1.convertTo(I1, CV_32F);
    i2.convertTo(I2, CV_32F);
    cv::Mat I1_2 = I1.mul(I1), I2_2 = I2.mul(I2), I1_I2 = I1.mul(I2);
    cv::Mat mu1, mu2;
    cv::GaussianBlur(I1, mu1, cv::Size(11, 11), 1.5);
    cv::GaussianBlur(I2, mu2, cv::Size(11, 11), 1.5);
    cv::Mat mu1_2 = mu1.mul(mu1), mu2_2 = mu2.mul(mu2), mu1_mu2 = mu1.mul(mu2);
    cv::Mat sigma1_2, sigma2_2, sigma12;
    cv::GaussianBlur(I1_2, sigma1_2, cv::Size(11, 11), 1.5); sigma1_2 -= mu1_2;
    cv::GaussianBlur(I2_2, sigma2_2, cv::Size(11, 11), 1.5); sigma2_2 -= mu2_2;
    cv::GaussianBlur(I1_I2, sigma12, cv::Size(11, 11), 1.5); sigma12 -= mu1_mu2;
    cv::Mat t1 = 2 * mu1_mu2 + C1, t2 = 2 * sigma12 + C2;
    cv::Mat t3 = t1.mul(t2);
    t1 = mu1_2 + mu2_2 + C1;
    t2 = sigma1_2 + sigma2_2 + C2;
    t1 = t1.mul(t2);
    cv::Mat ssim_map;
    cv::divide(t3, t1, ssim_map);
    return cv::mean(ssim_map)[0];
}

int main(int argc, char** argv) {
    std::string assets = argc > 1 ? argv[1] : "../../snippets/assets";
    cv::setNumThreads(1);

    cv::Mat linux_logo = asset(assets, "LinuxLogo.jpg", cv::IMREAD_COLOR);
    cv::Mat win_logo = asset(assets, "WindowsLogo.jpg", cv::IMREAD_COLOR);
    cv::Mat coffee = asset(assets, "coffee.png", cv::IMREAD_COLOR);
    cv::Mat chelsea = asset(assets, "chelsea.png", cv::IMREAD_COLOR);
    cv::Mat chelsea_gray = asset(assets, "chelsea.png", cv::IMREAD_GRAYSCALE);
    cv::Mat coins = asset(assets, "coins.png", cv::IMREAD_GRAYSCALE);
    cv::Mat building = asset(assets, "building.jpg", cv::IMREAD_GRAYSCALE);
    cv::Mat astronaut = asset(assets, "astronaut.png", cv::IMREAD_COLOR);
    cv::Mat invoice = asset(assets, "invoice.png", cv::IMREAD_GRAYSCALE);
    cv::Mat invoice_photo = asset(assets, "invoice_photo.png", cv::IMREAD_COLOR);
    cv::Mat box_img = asset(assets, "box.png", cv::IMREAD_GRAYSCALE);
    cv::Mat box_scene = asset(assets, "box_in_scene.png", cv::IMREAD_GRAYSCALE);

    cv::Mat coins_mask;
    cv::threshold(coins, coins_mask, 0, 255, cv::THRESH_BINARY | cv::THRESH_OTSU);
    cv::Mat invoice_smooth, invoice_mask;
    cv::GaussianBlur(invoice, invoice_smooth, cv::Size(5, 5), 0.8);
    cv::threshold(invoice_smooth, invoice_mask, 128, 255, cv::THRESH_BINARY_INV);

    cv::Mat building_blur;
    cv::GaussianBlur(building, building_blur, cv::Size(0, 0), 2.0);
    cv::Mat chelsea_blur;
    cv::GaussianBlur(chelsea_gray, chelsea_blur, cv::Size(0, 0), 1.5);
    chelsea_blur += 25;
    cv::Mat tmpl = chelsea_gray(cv::Rect((chelsea_gray.cols - 40) / 2, chelsea_gray.rows * 2 / 5 - 20, 40, 40)).clone();

    std::vector<Row> rows;
    std::vector<Digest> digests;
    cv::Mat dst;
    // For ops whose result is a list rather than `dst`: replaces the image digest with the element count.
    auto set_count_digest = [&](const char* id, size_t count) {
        digests.erase(std::remove_if(digests.begin(), digests.end(), [&](const Digest& d) { return d.id == id; }), digests.end());
        digests.push_back({id, static_cast<int>(count), 1, 0.0});
    };
    // Times each op twice: reusing `dst` (the OpenCV idiom) and releasing it first so every call
    // allocates its output like the Rust snippets do. The last fresh run also yields the digest.
    auto push = [&](const char* id, int iters, auto fn) {
        rows.push_back({id, bench_ms(3, iters, fn)});
        auto fresh = [&] { dst.release(); fn(); };
        rows.push_back({std::string(id) + "__fresh", bench_ms(3, iters, fresh)});
        dst.release();
        fn();
        if (!dst.empty() && dst.depth() == CV_8U) {
            cv::Scalar m = cv::mean(dst);
            double sum = 0.0;
            for (int c = 0; c < dst.channels(); ++c) sum += m[c];
            digests.push_back({id, dst.cols, dst.rows, sum / dst.channels()});
        }
    };
    push("adding_images", 50, [&] { cv::addWeighted(linux_logo, 0.5, win_logo, 0.5, 0.0, dst); });
    push("basic_linear_transform", 50, [&] { coffee.convertTo(dst, -1, 1.3, 40.0); });

    push("smoothing_box", 30, [&] { cv::blur(chelsea, dst, cv::Size(5, 5)); });
    push("smoothing_gaussian", 30, [&] { cv::GaussianBlur(chelsea, dst, cv::Size(5, 5), 1.1); });
    push("smoothing_median", 15, [&] { cv::medianBlur(chelsea, dst, 5); });
    push("smoothing_bilateral", 10, [&] { cv::bilateralFilter(chelsea, dst, 5, 30.0, 30.0); });

    cv::Mat k5 = cv::getStructuringElement(cv::MORPH_RECT, cv::Size(5, 5));
    cv::Mat k7 = cv::getStructuringElement(cv::MORPH_RECT, cv::Size(7, 7));
    push("erosion", 50, [&] { cv::erode(coins_mask, dst, k5); });
    push("equalize", 50, [&] { cv::equalizeHist(building, dst); });
    push("template_matching", 20, [&] { cv::matchTemplate(chelsea_blur, tmpl, dst, cv::TM_SQDIFF); });
    push("canny", 30, [&] { cv::GaussianBlur(building, dst, cv::Size(0, 0), 1.4); cv::Canny(dst, dst, 50.0, 150.0); });
    push("distance_transform", 30, [&] { cv::distanceTransform(coins_mask, dst, cv::DIST_L2, 3); });

    cv::Mat box_k = cv::Mat::ones(5, 5, CV_32F) / 25.0f;
    push("filter_2d", 30, [&] { cv::filter2D(chelsea, dst, -1, box_k); });

    std::vector<cv::Vec3f> circles;
    push("hough_circle", 15, [&] { cv::HoughCircles(coins, circles, cv::HOUGH_GRADIENT, 1.0, 20.0, 150.0, 25.0, 15, 40); });

    std::vector<cv::Vec2f> lines;
    push("hough_lines", 20, [&] { cv::Canny(building, dst, 50.0, 150.0); cv::HoughLines(dst, lines, 1.0, CV_PI / 180.0, 150); });
    set_count_digest("hough_lines", lines.size());

    push("laplace", 30, [&] {
        cv::Mat g, l;
        cv::GaussianBlur(building, g, cv::Size(0, 0), 1.0);
        cv::Laplacian(g, l, CV_16S, 3);
        cv::convertScaleAbs(l, dst);
    });
    push("sobel", 30, [&] {
        cv::Mat gx;
        cv::Sobel(building, gx, CV_16S, 1, 0, 3);
        cv::convertScaleAbs(gx, dst);
    });

    cv::Mat rot = cv::getRotationMatrix2D(cv::Point2f(astronaut.cols / 2.0f, astronaut.rows / 2.0f), -30.0, 0.6);
    push("warp_affine", 30, [&] { cv::warpAffine(astronaut, dst, rot, astronaut.size(), cv::INTER_LINEAR); });
    push("opening_closing_hats", 30, [&] { cv::morphologyEx(coins, dst, cv::MORPH_TOPHAT, k7); });
    push("pyramids", 30, [&] {
        cv::Mat p1, p2, p3;
        cv::pyrDown(chelsea, p1);
        cv::pyrDown(p1, p2);
        cv::pyrDown(p2, p3);
        dst = p3;
    });

    push("find_contours", 20, [&] {
        std::vector<std::vector<cv::Point>> contours;
        cv::findContours(invoice_mask.clone(), contours, cv::RETR_LIST, cv::CHAIN_APPROX_SIMPLE);
        std::vector<cv::Point> poly;
        for (const auto& c : contours) cv::approxPolyDP(c, poly, 2.0, true);
    });
    push("threshold", 50, [&] { cv::threshold(coins, dst, 0, 255, cv::THRESH_BINARY | cv::THRESH_OTSU); });
    push("threshold_inRange", 30, [&] {
        cv::Mat hsv, m1, m2;
        cv::cvtColor(win_logo, hsv, cv::COLOR_BGR2HSV_FULL);
        cv::inRange(hsv, cv::Scalar(244, 128, 102), cv::Scalar(255, 255, 255), m1);
        cv::inRange(hsv, cv::Scalar(0, 128, 102), cv::Scalar(11, 255, 255), m2);
        cv::bitwise_or(m1, m2, dst);
    });

    auto fast = cv::FastFeatureDetector::create(30, true);
    std::vector<cv::KeyPoint> kps;
    push("feature_detection_fast", 30, [&] { kps.clear(); fast->detect(building, kps); });
    digests.push_back({"feature_detection_fast", static_cast<int>(kps.size()), 1, 0.0});

    auto orb = cv::ORB::create(250);
    cv::Mat desc;
    push("feature_detection_orb", 20, [&] { kps.clear(); orb->detectAndCompute(building, cv::noArray(), kps, desc); });
    digests.push_back({"feature_detection_orb", static_cast<int>(kps.size()), 1, 0.0});
    push("feature_description", 10, [&] {
        std::vector<cv::KeyPoint> ka, kb;
        cv::Mat da, db;
        orb->detectAndCompute(box_img, cv::noArray(), ka, da);
        orb->detectAndCompute(box_scene, cv::noArray(), kb, db);
        std::vector<cv::DMatch> matches;
        if (!da.empty() && !db.empty()) {
            cv::BFMatcher(cv::NORM_HAMMING).match(da, db, matches);
        }
    });

    std::vector<cv::Point2f> src_pts = {{70.0f, 40.0f}, {580.0f, 70.0f}, {555.0f, 430.0f}, {40.0f, 400.0f}};
    std::vector<cv::Point2f> dst_pts = {{0.0f, 0.0f}, {640.0f, 0.0f}, {640.0f, 480.0f}, {0.0f, 480.0f}};
    push("homography", 30, [&] {
        cv::Mat m = cv::getPerspectiveTransform(src_pts, dst_pts);
        cv::warpPerspective(invoice_photo, dst, m, invoice_photo.size(), cv::INTER_LINEAR);
    });

    volatile double sink = 0.0;
    push("psnr", 50, [&] { sink = opencv_psnr(building, building_blur); });
    push("ssim", 20, [&] { sink = opencv_ssim(building, building_blur); });
    (void)sink;

    std::cout << "{\n  \"opencv_version\": \"" << CV_VERSION << "\",\n  \"build_info\": [";
    std::vector<std::string> info = build_lines();
    for (size_t i = 0; i < info.size(); ++i) {
        std::cout << (i ? ", " : "") << "\"" << json_escape(info[i]) << "\"";
    }
    std::cout << "],\n  \"digests\": {\n";
    for (size_t i = 0; i < digests.size(); ++i) {
        std::cout << "    \"" << digests[i].id << "\": {\"w\": " << digests[i].w << ", \"h\": " << digests[i].h
                  << ", \"mean\": " << digests[i].mean << "}" << (i + 1 < digests.size() ? "," : "") << "\n";
    }
    std::cout << "  },\n  \"results\": {\n";
    for (size_t i = 0; i < rows.size(); ++i) {
        std::cout << "    \"" << rows[i].id << "\": " << rows[i].ms
                  << (i + 1 < rows.size() ? "," : "") << "\n";
    }
    std::cout << "  }\n}\n";
    return 0;
}
