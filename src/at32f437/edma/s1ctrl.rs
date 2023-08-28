#[doc = "Register `S1CTRL` reader"]
pub type R = crate::R<S1CTRL_SPEC>;
#[doc = "Register `S1CTRL` writer"]
pub type W = crate::W<S1CTRL_SPEC>;
#[doc = "Field `SEN` reader - Stream enable / flag stream ready when read low"]
pub type SEN_R = crate::BitReader;
#[doc = "Field `SEN` writer - Stream enable / flag stream ready when read low"]
pub type SEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMERRIEN` reader - Direct mode error interrupt enable"]
pub type DMERRIEN_R = crate::BitReader;
#[doc = "Field `DMERRIEN` writer - Direct mode error interrupt enable"]
pub type DMERRIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTERRIEN` reader - Transfer error interrupt enable"]
pub type DTERRIEN_R = crate::BitReader;
#[doc = "Field `DTERRIEN` writer - Transfer error interrupt enable"]
pub type DTERRIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDTIEN` reader - Half data transfer interrupt enable"]
pub type HDTIEN_R = crate::BitReader;
#[doc = "Field `HDTIEN` writer - Half data transfer interrupt enable"]
pub type HDTIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FDTIEN` reader - Full data transfer complete interrupt enable"]
pub type FDTIEN_R = crate::BitReader;
#[doc = "Field `FDTIEN` writer - Full data transfer complete interrupt enable"]
pub type FDTIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PFCTRL` reader - Peripheral flow controller"]
pub type PFCTRL_R = crate::BitReader;
#[doc = "Field `PFCTRL` writer - Peripheral flow controller"]
pub type PFCTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTD` reader - Data transfer direction"]
pub type DTD_R = crate::FieldReader;
#[doc = "Field `DTD` writer - Data transfer direction"]
pub type DTD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LM` reader - Loop mode"]
pub type LM_R = crate::BitReader;
#[doc = "Field `LM` writer - Loop mode"]
pub type LM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PINCM` reader - Peripheral increment mode"]
pub type PINCM_R = crate::BitReader;
#[doc = "Field `PINCM` writer - Peripheral increment mode"]
pub type PINCM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MINCM` reader - Memory increment mode"]
pub type MINCM_R = crate::BitReader;
#[doc = "Field `MINCM` writer - Memory increment mode"]
pub type MINCM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWIDTH` reader - Peripheral data width"]
pub type PWIDTH_R = crate::FieldReader;
#[doc = "Field `PWIDTH` writer - Peripheral data width"]
pub type PWIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MWIDTH` reader - Memory data width"]
pub type MWIDTH_R = crate::FieldReader;
#[doc = "Field `MWIDTH` writer - Memory data width"]
pub type MWIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PINCOS` reader - Peripheral increment offset size"]
pub type PINCOS_R = crate::BitReader;
#[doc = "Field `PINCOS` writer - Peripheral increment offset size"]
pub type PINCOS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPL` reader - Stream priority level"]
pub type SPL_R = crate::FieldReader;
#[doc = "Field `SPL` writer - Stream priority level"]
pub type SPL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DMM` reader - Double memory mode"]
pub type DMM_R = crate::BitReader;
#[doc = "Field `DMM` writer - Double memory mode"]
pub type DMM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CM` reader - Current memory (only in double buffer mode)"]
pub type CM_R = crate::BitReader;
#[doc = "Field `CM` writer - Current memory (only in double buffer mode)"]
pub type CM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PBURST` reader - Peripheral burst transmission"]
pub type PBURST_R = crate::FieldReader;
#[doc = "Field `PBURST` writer - Peripheral burst transmission"]
pub type PBURST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MBURST` reader - Memory burst transmission"]
pub type MBURST_R = crate::FieldReader;
#[doc = "Field `MBURST` writer - Memory burst transmission"]
pub type MBURST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 0 - Stream enable / flag stream ready when read low"]
    #[inline(always)]
    pub fn sen(&self) -> SEN_R {
        SEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direct mode error interrupt enable"]
    #[inline(always)]
    pub fn dmerrien(&self) -> DMERRIEN_R {
        DMERRIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn dterrien(&self) -> DTERRIEN_R {
        DTERRIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half data transfer interrupt enable"]
    #[inline(always)]
    pub fn hdtien(&self) -> HDTIEN_R {
        HDTIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Full data transfer complete interrupt enable"]
    #[inline(always)]
    pub fn fdtien(&self) -> FDTIEN_R {
        FDTIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral flow controller"]
    #[inline(always)]
    pub fn pfctrl(&self) -> PFCTRL_R {
        PFCTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Data transfer direction"]
    #[inline(always)]
    pub fn dtd(&self) -> DTD_R {
        DTD_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Loop mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral increment mode"]
    #[inline(always)]
    pub fn pincm(&self) -> PINCM_R {
        PINCM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Memory increment mode"]
    #[inline(always)]
    pub fn mincm(&self) -> MINCM_R {
        MINCM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Peripheral data width"]
    #[inline(always)]
    pub fn pwidth(&self) -> PWIDTH_R {
        PWIDTH_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Memory data width"]
    #[inline(always)]
    pub fn mwidth(&self) -> MWIDTH_R {
        MWIDTH_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Peripheral increment offset size"]
    #[inline(always)]
    pub fn pincos(&self) -> PINCOS_R {
        PINCOS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Stream priority level"]
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Double memory mode"]
    #[inline(always)]
    pub fn dmm(&self) -> DMM_R {
        DMM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Current memory (only in double buffer mode)"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Peripheral burst transmission"]
    #[inline(always)]
    pub fn pburst(&self) -> PBURST_R {
        PBURST_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Memory burst transmission"]
    #[inline(always)]
    pub fn mburst(&self) -> MBURST_R {
        MBURST_R::new(((self.bits >> 23) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Stream enable / flag stream ready when read low"]
    #[inline(always)]
    #[must_use]
    pub fn sen(&mut self) -> SEN_W<S1CTRL_SPEC, 0> {
        SEN_W::new(self)
    }
    #[doc = "Bit 1 - Direct mode error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmerrien(&mut self) -> DMERRIEN_W<S1CTRL_SPEC, 1> {
        DMERRIEN_W::new(self)
    }
    #[doc = "Bit 2 - Transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dterrien(&mut self) -> DTERRIEN_W<S1CTRL_SPEC, 2> {
        DTERRIEN_W::new(self)
    }
    #[doc = "Bit 3 - Half data transfer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hdtien(&mut self) -> HDTIEN_W<S1CTRL_SPEC, 3> {
        HDTIEN_W::new(self)
    }
    #[doc = "Bit 4 - Full data transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdtien(&mut self) -> FDTIEN_W<S1CTRL_SPEC, 4> {
        FDTIEN_W::new(self)
    }
    #[doc = "Bit 5 - Peripheral flow controller"]
    #[inline(always)]
    #[must_use]
    pub fn pfctrl(&mut self) -> PFCTRL_W<S1CTRL_SPEC, 5> {
        PFCTRL_W::new(self)
    }
    #[doc = "Bits 6:7 - Data transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn dtd(&mut self) -> DTD_W<S1CTRL_SPEC, 6> {
        DTD_W::new(self)
    }
    #[doc = "Bit 8 - Loop mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<S1CTRL_SPEC, 8> {
        LM_W::new(self)
    }
    #[doc = "Bit 9 - Peripheral increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn pincm(&mut self) -> PINCM_W<S1CTRL_SPEC, 9> {
        PINCM_W::new(self)
    }
    #[doc = "Bit 10 - Memory increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn mincm(&mut self) -> MINCM_W<S1CTRL_SPEC, 10> {
        MINCM_W::new(self)
    }
    #[doc = "Bits 11:12 - Peripheral data width"]
    #[inline(always)]
    #[must_use]
    pub fn pwidth(&mut self) -> PWIDTH_W<S1CTRL_SPEC, 11> {
        PWIDTH_W::new(self)
    }
    #[doc = "Bits 13:14 - Memory data width"]
    #[inline(always)]
    #[must_use]
    pub fn mwidth(&mut self) -> MWIDTH_W<S1CTRL_SPEC, 13> {
        MWIDTH_W::new(self)
    }
    #[doc = "Bit 15 - Peripheral increment offset size"]
    #[inline(always)]
    #[must_use]
    pub fn pincos(&mut self) -> PINCOS_W<S1CTRL_SPEC, 15> {
        PINCOS_W::new(self)
    }
    #[doc = "Bits 16:17 - Stream priority level"]
    #[inline(always)]
    #[must_use]
    pub fn spl(&mut self) -> SPL_W<S1CTRL_SPEC, 16> {
        SPL_W::new(self)
    }
    #[doc = "Bit 18 - Double memory mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmm(&mut self) -> DMM_W<S1CTRL_SPEC, 18> {
        DMM_W::new(self)
    }
    #[doc = "Bit 19 - Current memory (only in double buffer mode)"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<S1CTRL_SPEC, 19> {
        CM_W::new(self)
    }
    #[doc = "Bits 21:22 - Peripheral burst transmission"]
    #[inline(always)]
    #[must_use]
    pub fn pburst(&mut self) -> PBURST_W<S1CTRL_SPEC, 21> {
        PBURST_W::new(self)
    }
    #[doc = "Bits 23:24 - Memory burst transmission"]
    #[inline(always)]
    #[must_use]
    pub fn mburst(&mut self) -> MBURST_W<S1CTRL_SPEC, 23> {
        MBURST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "stream 1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s1ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s1ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S1CTRL_SPEC;
impl crate::RegisterSpec for S1CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s1ctrl::R`](R) reader structure"]
impl crate::Readable for S1CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s1ctrl::W`](W) writer structure"]
impl crate::Writable for S1CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S1CTRL to value 0"]
impl crate::Resettable for S1CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
